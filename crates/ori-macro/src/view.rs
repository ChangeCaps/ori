use manyhow::bail;
use proc_macro2::{Ident, TokenStream};
use quote::{quote, quote_spanned};
use rstml::node::{KeyedAttribute, Node, NodeAttribute, NodeName};
use syn::{
    parse::{discouraged::Speculative, Parse, ParseStream, Parser},
    parse_quote, parse_quote_spanned,
    spanned::Spanned,
    Expr, ExprPath, Path, Stmt, Token,
};

use crate::krate::find_crate;

enum Content {
    For(Expr),
    Expr(Expr),
}

impl Parse for Content {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let fork = input.fork();
        if fork.parse::<Token![for]>().is_ok() {
            let expr = fork.parse::<Expr>()?;
            input.advance_to(&fork);
            Ok(Self::For(expr))
        } else {
            let expr = input.parse::<Expr>()?;
            Ok(Self::Expr(expr))
        }
    }
}

/// Transforms a block into an iterator.
///
/// This introduces the `for` keyword to the view macro.
///
/// # Example
/// ```ignore
/// let items = vec![1, 2, 3];
///
/// view! {
///     <Div>
///         { for item }
///     </Div>
/// }
/// ```
fn transform_block(parser: ParseStream) -> Result<Option<TokenStream>, syn::Error> {
    let ori_core = find_crate("core");

    let content = parser.parse::<Content>()?;

    let tokens = match content {
        Content::For(expr) => quote!(::std::iter::IntoIterator::into_iter(#expr)),
        Content::Expr(expr) => quote!(::std::iter::once(#ori_core::View::new(#expr))),
    };

    Ok(Some(tokens))
}

pub fn view(input: proc_macro::TokenStream) -> manyhow::Result<proc_macro::TokenStream> {
    let (context, rest) = parse_context(input.into())?;

    let config = rstml::ParserConfig::new().transform_block(transform_block);
    let parser = rstml::Parser::new(config);

    let nodes = if !rest.is_empty() {
        parser.parse_simple(rest)?
    } else {
        Vec::new()
    };

    let expanded = if nodes.len() == 1 {
        create_node(&context, nodes.iter())?
    } else {
        create_fragment(&context, nodes.iter())?
    };

    Ok(expanded.into())
}

// parses
// $expr , $rest | $rest
fn parse_context(input: TokenStream) -> manyhow::Result<(Expr, TokenStream)> {
    let parser = |parser: ParseStream| {
        if parser.is_empty() {
            return Ok((parse_quote!(cx), TokenStream::new()));
        }

        let fork = parser.fork();
        let Ok(expr) = fork.parse::<Expr>() else {
            return Ok((parse_quote!(cx), parser.parse()?));
        };

        if fork.peek(Token![,]) {
            fork.parse::<Token![,]>()?;
        } else {
            return Ok((parse_quote!(cx), parser.parse()?));
        }

        parser.advance_to(&fork);
        Ok((expr, parser.parse()?))
    };

    Ok(Parser::parse2(parser, input)?)
}

/// Creates an element node.
fn create_node<'a>(
    context: &Expr,
    nodes: impl Iterator<Item = &'a Node>,
) -> manyhow::Result<TokenStream> {
    let ori_core = find_crate("core");

    let mut elements = Vec::new();

    for node in nodes {
        elements.push(view_node(context, node)?);
    }

    Ok(quote! {
        #(#ori_core::View::new(#elements))*
    })
}

/// Creates a fragment node.
fn create_fragment<'a>(
    context: &Expr,
    nodes: impl Iterator<Item = &'a Node>,
) -> manyhow::Result<TokenStream> {
    let ori_core = find_crate("core");

    let mut elements = Vec::new();

    for node in nodes {
        let node = view_node(context, node)?;
        elements.push(quote! {
            __views.push(#node);
        });
    }

    Ok(quote! {{
        let mut __views = ::std::vec::Vec::new();
        #(#elements)*
        #ori_core::View::fragment(__views)
    }})
}

/// Adds a child to an `__node` statically.
fn static_child(name: &Path, child: &Expr) -> TokenStream {
    let ori_core = find_crate("core");

    quote_spanned! {child.span() =>
        let __node = __view.get_node().unwrap();
        let _ = __node.downcast::<#name, ()>(move |__view| {
            <#name as #ori_core::Parent>::add_child(__view, #child);
        });
    }
}

/// Adds children to an `__node`.
fn children<'a>(
    context: &'a Expr,
    name: Path,
    children: impl Iterator<Item = &'a Node> + 'a,
) -> manyhow::Result<Vec<TokenStream>> {
    let mut vec = Vec::new();

    for node in children {
        let child = view_node(context, node)?;
        vec.push(static_child(&name, &child));
    }

    Ok(vec)
}

/// Returns an expression that evaluates to a `Node`.
fn view_node(context: &Expr, node: &Node) -> manyhow::Result<Expr> {
    let ori_core = find_crate("core");

    match node {
        Node::Element(element) => {
            let node_name = &element.name();
            let name = parse_quote!(#node_name);

            let mut attributes = Vec::new();
            let mut properties = Vec::new();

            for node in element.attributes() {
                let attr = get_keyed(node);
                attribute(context, &name, attr, &mut attributes, &mut properties)?;
            }

            let children = children(context, parse_quote!(#name), element.children.iter())?;

            Ok(
                parse_quote_spanned! {element.name().span() => #[allow(clippy::let_and_return)] {
                    let __view = <#name as #ori_core::Build>::build();

                    #(#children)*
                    #(#properties)*
                    #(#attributes)*

                    __view
                }},
            )
        }
        Node::Block(block) => {
            let block = block.try_block().unwrap();
            let inner_expr = match block.stmts.first().unwrap() {
                Stmt::Expr(Expr::Call(expr), _) => expr.args.first().unwrap(),
                _ => unreachable!(),
            };
            let dynamic = expr_is_dynamic(inner_expr);

            let fragment = parse_quote_spanned!(block.span() =>
                #[allow(unused_braces)]
                #ori_core::View::fragment(::std::iter::Iterator::collect::<::std::vec::Vec<_>>(#block))
            );

            if dynamic {
                Ok(parse_quote_spanned! {block.span() => {
                    let __view = #context.owned_memo_scoped(move |#context| {
                        #context.emit(#ori_core::RequestRedrawEvent);
                        #fragment
                    });
                    #ori_core::View::dynamic(__view)
                }})
            } else {
                Ok(fragment)
            }
        }
        Node::Comment(comment) => {
            let comment = &comment.value;

            Ok(parse_quote_spanned! {comment.span() =>
                #ori_core::View::new(#ori_core::Comment::new(#comment))
            })
        }
        Node::Text(text) => {
            let text = &text.value;

            Ok(parse_quote_spanned! {text.span() =>
                #ori_core::View::new(#text)
            })
        }
        _ => unreachable!(),
    }
}

fn get_keyed(node: &NodeAttribute) -> &KeyedAttribute {
    let NodeAttribute::Attribute(keyed) = node else {
        unreachable!()
    };

    keyed
}

fn attribute(
    context: &Expr,
    name: &Ident,
    attr: &KeyedAttribute,
    attributes: &mut Vec<TokenStream>,
    properties: &mut Vec<TokenStream>,
) -> manyhow::Result<()> {
    if let NodeName::Path(ref path) = attr.key {
        let Some(value) = attr.value() else { return Ok(()) };

        if path.path == parse_quote!(class) {
            attributes.push(class(context, name, value));
            return Ok(());
        }

        if path.path.get_ident().map_or(false, |ident| ident == "ref") {
            attributes.push(node_ref(context, name, value));
            return Ok(());
        }

        properties.push(property(context, name, path, value));
        return Ok(());
    }

    if let NodeName::Punctuated(ref punct) = attr.key {
        let (kind, key) = attribute_kind(attr)?;

        match kind.as_str() {
            "on" => {
                if let Some(value) = attr.value() {
                    let key = Ident::new(&key, punct.span());
                    properties.push(event(context, name, &key, value));
                } else {
                    bail!(punct, "expected event handler");
                }
            }
            "bind" => {
                if let Some(value) = attr.value() {
                    let key = Ident::new(&key, punct.span());
                    properties.push(binding(context, name, &key, value));
                } else {
                    bail!(punct, "expected binding");
                }
            }
            "style" => {
                if let Some(value) = attr.value() {
                    attributes.push(style(context, name, &key, value));
                } else {
                    bail!(punct, "expected attribute");
                }
            }
            _ => bail!(kind, "invalid attribute kind"),
        }
    }

    Ok(())
}

fn attribute_kind(attribute: &KeyedAttribute) -> manyhow::Result<(String, String)> {
    let NodeName::Punctuated(ref punct) = attribute.key else {
        unreachable!()
    };

    let mut pairs = punct.pairs();

    let pair = pairs.next().unwrap();
    let kind = pair.value();

    if pair.punct().unwrap().as_char() != ':' {
        bail!(punct, "expected ':'");
    }

    let mut key = String::new();
    for pair in pairs {
        let ident = pair.value();

        key.push_str(&ident.to_string());

        if let Some(punct) = pair.punct() {
            if punct.as_char() != '-' {
                bail!(punct, "expected '-'");
            }

            key.push('-');
        }
    }

    if key.is_empty() {
        bail!(punct, "expected attribute name");
    }

    Ok((kind.to_string(), key))
}

/// Checks if the given expression is dynamic, which means that it can change reactively.
fn expr_is_dynamic(value: &Expr) -> bool {
    match value {
        Expr::Array(expr) => expr.elems.iter().any(expr_is_dynamic),
        Expr::Assign(_) => false,
        Expr::Block(expr) => expr.block.stmts.iter().any(|stmt| match stmt {
            Stmt::Expr(expr, _) => expr_is_dynamic(expr),
            _ => true,
        }),
        Expr::Unary(expr) => expr_is_dynamic(&expr.expr),
        Expr::Binary(expr) => expr_is_dynamic(&expr.left) || expr_is_dynamic(&expr.right),
        Expr::Cast(expr) => expr_is_dynamic(&expr.expr),
        Expr::Closure(_) => false,
        Expr::Field(expr) => expr_is_dynamic(&expr.base),
        Expr::Group(expr) => expr_is_dynamic(&expr.expr),
        Expr::Lit(_) => false,
        Expr::Paren(expr) => expr_is_dynamic(&expr.expr),
        Expr::Path(_) => false,
        Expr::Reference(expr) => expr_is_dynamic(&expr.expr),
        Expr::Repeat(expr) => expr_is_dynamic(&expr.expr),
        Expr::Return(expr) => match expr.expr.as_ref() {
            Some(expr) => expr_is_dynamic(expr),
            None => false,
        },
        Expr::Try(expr) => expr_is_dynamic(&expr.expr),
        Expr::Tuple(expr) => expr.elems.iter().any(expr_is_dynamic),
        _ => true,
    }
}

/// Wraps the given block in an effect scope.
fn wrap_dynamic(context: &Expr, value: TokenStream) -> TokenStream {
    let ori_core = find_crate("core");

    quote! {
        #context.effect_scoped({
            let __view = __view.clone();
            move |#context| {
                #value
                #context.emit(#ori_core::RequestRedrawEvent);
            }
        });
    }
}

/// Sets the class of the given `__node`.
fn class(context: &Expr, _name: &Ident, value: &Expr) -> TokenStream {
    let ori_core = find_crate("core");

    let set_scope = quote_spanned! {value.span() =>
        #ori_core::View::visit(&__view, move |__node| {
            __node.node_state().style.set_class(#value);
            __node.request_layout();
        });
    };

    if expr_is_dynamic(value) {
        wrap_dynamic(context, set_scope)
    } else {
        set_scope
    }
}

/// Sets the given NodeRef to `__node`.
fn node_ref(context: &Expr, _name: &Ident, value: &Expr) -> TokenStream {
    let ori_core = find_crate("core");

    let set_node_ref = quote_spanned! {value.span() =>
        let __node = __view.get_node().unwrap();
        #ori_core::NodeRef::set(&#value, __node.clone());
    };

    if expr_is_dynamic(value) {
        wrap_dynamic(context, set_node_ref)
    } else {
        set_node_ref
    }
}

/// Sets a style attribute of the given `__node`.
fn style(context: &Expr, _name: &Ident, key: &str, value: &Expr) -> TokenStream {
    let set_style = quote_spanned! {value.span() =>
        let __node = __view.get_node().unwrap();
        __node.node_state().style.set_attr(#key, #value);
        __node.request_layout();
    };

    if expr_is_dynamic(value) {
        wrap_dynamic(context, set_style)
    } else {
        set_style
    }
}

/// Sets a property of the given `__node`.
fn property(context: &Expr, name: &Ident, key: &ExprPath, value: &Expr) -> TokenStream {
    let ori_core = find_crate("core");

    let key = quote_spanned! {key.path.span() =>
        #key
    };

    let set_property = quote_spanned! {value.span() =>
        let __node = __view.get_node().unwrap();
        __node.downcast::<#name, _>(|__node| {
            <#name as #ori_core::Build>::prop(__node).#key(#value);
        }).unwrap();
    };

    if expr_is_dynamic(value) {
        wrap_dynamic(context, set_property)
    } else {
        set_property
    }
}

/// Sets an event handler of the given `__node`.
fn event(context: &Expr, name: &Ident, key: &Ident, value: &Expr) -> TokenStream {
    let ori_core = find_crate("core");

    let set_event = quote_spanned! {value.span() =>
        let __node = __view.get_node().unwrap();
        __node.downcast::<#name, _>(|__node| {
            <#name as #ori_core::Build>::on(__node).#key(#context, #value);
        }).unwrap();
    };

    if expr_is_dynamic(value) {
        wrap_dynamic(context, set_event)
    } else {
        set_event
    }
}

/// Sets a binding of the given `__node`.
fn binding(context: &Expr, name: &Ident, key: &Ident, value: &Expr) -> TokenStream {
    let ori_core = find_crate("core");

    let set_binding = quote_spanned! {value.span() =>
        let __node = __view.get_node().unwrap();
        __node.downcast::<#name, _>(|__node| {
            <#name as #ori_core::Build>::bind(__node).#key(#value);
        }).unwrap();
    };

    if expr_is_dynamic(value) {
        wrap_dynamic(context, set_binding)
    } else {
        set_binding
    }
}
