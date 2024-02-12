extern crate proc_macro;
use proc_macro2::{Delimiter, Group, Ident, Punct, Spacing, TokenStream, TokenTree, Span, Literal};
use proc_macro_error::{abort, proc_macro_error};
use quote::quote;
use std::borrow::Cow;

/// Use short hand notation to describe a key chord; returns a tuple of
/// `(modifiers, key_code)`.
///
/// Specify a key and any modifiers.
///
#[cfg_attr(not(any(feature = "winit", feature = "bevy")), doc = r##"
Paragraph that only appears when feature is enabled.

```
# use keyseq_macro::key;
assert_eq!(key!(A), (0, "A"));
assert_eq!(key!(shift-A), (1, "A"));
assert_eq!(key!(ctrl-A), (2, "A"));
assert_eq!(key!(alt-A), (4, "A"));
assert_eq!(key!(super-A), (8, "A"));
assert_eq!(key!(alt-ctrl-;), (6, "Semicolon"));
assert_eq!(key!(1), (0, "Key1"));
assert_eq!(key!(alt-1), (4, "Key1"));
```
"##)]
#[cfg_attr(feature = "bevy", doc = r##"
Paragraph that only appears when feature is enabled.

```
# use keyseq_macro::key;
use bevy::prelude::KeyCode;
assert_eq!(key!(A), (0, KeyCode::A));
assert_eq!(key!(shift-A), (1, KeyCode::A));
assert_eq!(key!(ctrl-A), (2, KeyCode::A));
assert_eq!(key!(alt-A), (4, KeyCode::A));
assert_eq!(key!(super-A), (8, KeyCode::A));
assert_eq!(key!(shift-ctrl-A), (3, KeyCode::A));
assert_eq!(key!(alt-ctrl-;), (6, KeyCode::Semicolon));
assert_eq!(key!(1), (0, KeyCode::Key1));
assert_eq!(key!(alt-1), (4, KeyCode::Key1));
```
"##)]
/// Can use symbols or their given name in KeyCode enum, e.g. ';' or "Semicolon".
///
/// ```ignore
/// assert_eq!(key!(ctrl-;), (Modifiers::Control, KeyCode::Semicolon));
/// assert_eq!(key!(ctrl-Semicolon), (Modifiers::Control, KeyCode::Semicolon));
/// ```
///
/// More than one key will cause a panic at compile-time. Use keyseq! for that.
///
/// ```ignore
/// fn too_many_keys() {
///     let _ = key!(A B);
/// }
/// ```
#[proc_macro_error]
#[proc_macro]
pub fn key(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let (result, leftover) = partial_key_code(input.into());
    if !leftover.is_empty() {
        abort!(leftover, "Too many tokens; use keyseq! for multiple keys");
    }
    result.into()
}

/// Use short hand notation to describe a logical key chord; returns a tuple of
/// `(modifiers, key_code)`.
///
/// Specify a key and any modifiers.
///
#[cfg_attr(not(feature = "winit"), doc = r##"
```
# use keyseq_macro::lkey;
assert_eq!(lkey!(a), (0, "a"));
assert_eq!(lkey!(A), (0, "A"));
assert_eq!(lkey!(shift-A), (1, "A"));
assert_eq!(lkey!(ctrl-A), (2, "A"));
assert_eq!(lkey!(alt-A), (4, "A"));
assert_eq!(lkey!(super-A), (8, "A"));
assert_eq!(lkey!(alt-ctrl-;), (6, ";"));
assert_eq!(lkey!(1), (0, "1"));
assert_eq!(lkey!(alt-1), (4, "1"));
```
"##)]
///
/// Can use symbols or their given name in KeyCode enum, e.g. ';' or "Semicolon".
///
/// ```ignore
/// assert_eq!(key!(ctrl-;), (Modifiers::Control, KeyCode::Semicolon));
/// assert_eq!(key!(ctrl-Semicolon), (Modifiers::Control, KeyCode::Semicolon));
/// ```
///
/// More than one key will cause a panic at compile-time. Use keyseq! for that.
///
/// ```compile_fail
/// fn too_many_keys() {
///     let _ = key!(A B);
/// }
/// ```
#[cfg_attr(feature = "winit", doc = r##"
Paragraph that only appears when feature is enabled.

```
# use keyseq_macro::lkey;
use winit::keyboard::{ModifiersState, Key};
assert_eq!(lkey!(a), (ModifiersState::empty(), Key::Character('a')));
assert_eq!(lkey!(A), (ModifiersState::empty(), Key::Character('A')));
assert_eq!(lkey!(shift-A), (ModifiersState::SHIFT, Key::Character('A')));
assert_eq!(lkey!(shift-a), (ModifiersState::SHIFT, Key::Character('a')));
assert_eq!(lkey!(ctrl-A), (ModifiersState::CONTROL, Key::Character('A')));
assert_eq!(lkey!(alt-A), (ModifiersState::ALT, Key::Character('A')));
assert_eq!(lkey!(super-A), (ModifiersState::SUPER, Key::Character('A')));
assert_eq!(lkey!(alt-ctrl-;), (ModifiersState::ALT | ModifiersState::CONTROL, Key::Character(';')));
assert_eq!(lkey!(1), (ModifiersState::empty(), Key::Character('1')));
assert_eq!(lkey!(!), (ModifiersState::empty(), Key::Character('!')));
```
"##)]
#[proc_macro_error]
#[proc_macro]
pub fn lkey(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let (result, leftover) = partial_key(input.into());
    if !leftover.is_empty() {
        abort!(leftover, "Too many tokens; use keyseq! for multiple keys");
    }
    result.into()
}

/// Uses a short hand notation to describe a sequence of key chords, returns an
/// array of tuples `(modifiers, key_code)`.
///
/// Specify a key and any modifiers.
///
/// ```ignore
/// assert_eq!!(keyseq!(A B), [(Modifiers::empty(), KeyCode::A), (Modifiers::empty(), KeyCode::B)]);
/// assert_eq!!(keyseq!(ctrl-A B), [(Modifiers::Control, KeyCode::A), (Modifiers::empty(), KeyCode::B)]);
/// assert_eq!!(keyseq!(alt-ctrl-A Escape), [(Modifiers::Alt | Modifiers::Control, KeyCode::A), (Modifiers::empty(), KeyCode::Escape)]);
/// ```
///
/// One can use symbols or their given name in KeyCode enum, e.g. ';' or "Semicolon".
///
/// ```ignore
/// assert_eq!!(keyseq!(ctrl-;), [(Modifiers::Control, KeyCode::Semicolon)]);
/// assert_eq!!(keyseq!(ctrl-Semicolon), [(Modifiers::Control, KeyCode::Semicolon)]);
/// ```
#[proc_macro_error]
#[proc_macro]
pub fn keyseq(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let mut input: TokenStream = input.into();
    let mut keys = vec![];

    loop {
        let (result, leftover) = partial_key_code(input);
        keys.push(result);
        if leftover.is_empty() {
            break;
        }
        input = leftover;
    }
    quote! {
        [#(#keys),*]
    }
    .into()
}

fn key_path(id: Ident) -> TokenStream {
    #[cfg(any(feature = "bevy", feature = "bevy-input-sequence"))]
    let q = quote! { ::bevy::prelude::Key::#id };
    #[cfg(feature = "winit")]
    let q = quote! { ::winit::keyboard::Key::#id };
    #[cfg(not(any(feature = "winit", feature = "bevy", feature = "bevy-input-sequence")))]
    let q = {
        let x = format!("{}", id);
        let s = proc_macro2::Literal::string(&x);
        quote!{ #s }
    };
    q
}

fn key_code_path(id: Ident) -> TokenStream {
    #[cfg(any(feature = "bevy", feature = "bevy-input-sequence"))]
    let q = quote! { ::bevy::prelude::KeyCode::#id };
    #[cfg(feature = "winit")]
    let q = quote! { ::winit::keyboard::KeyCode::#id };
    #[cfg(not(any(feature = "winit", feature = "bevy", feature = "bevy-input-sequence")))]
    let q = {
        let x = format!("{}", id);
        let s = proc_macro2::Literal::string(&x);
        quote!{ #s }
    };
    q
}

enum Modifier {
    // Use same order as winit.
    None = 0,
    Shift = 1,
    Control = 2,
    Alt = 3,
    Super = 4,
}

#[allow(unused_variables)]
fn modifiers_id(id: TokenStream, modifier: Modifier) -> TokenStream {
    #[cfg(feature = "bevy-input-sequence")]
    let q = quote! { ::bevy_input_sequence::prelude::Modifiers::#id };
    #[cfg(feature = "winit")]
    let q = quote! { ::winit::keyboard::ModifiersState::#id };
    #[cfg(not(any(feature = "winit", feature = "bevy-input-sequence")))]
    let q = {
        let mut number = modifier as u8;
        if number != 0 {
            number = 1 << (number - 1);
            // number = 1 << (number - 1) * 3;
        }
        let x = proc_macro2::Literal::u8_suffixed(number);
        quote! { #x }
    };
    q
}

#[cfg(feature = "winit")]
fn get_key(tree: TokenTree) -> Option<TokenStream> {
    get_key_raw(tree).map(|r| match r {
        Ok(c) => {
            let l = Literal::character(c);
            quote! { ::winit::keyboard::Key::Character(#l) }
        },
        Err(cow) => {
            let i = Ident::new(&cow, Span::call_site());
            quote! { ::winit::keyboard::Key::Named(::winit::keyboard::NamedKey::#i) }
        }
    })
}

#[cfg(not(feature = "winit"))]
fn get_key(tree: TokenTree) -> Option<TokenStream> {
    get_key_raw(tree).map(|r| match r {
        Ok(c) => {
            let l = Literal::string(&c.to_string());
            quote! { #l }
        },
        Err(cow) => {
            let l = Literal::string(&cow);
            quote! { #l }
        }
    })
}

fn get_key_raw(tree: TokenTree) -> Option<Result<char, Cow<'static, str>>> {
    match tree {
        TokenTree::Literal(ref literal) => {
            let x = literal.span().source_text().unwrap();
            if x.len() == 1 {
                Some(Ok(x.chars().next().unwrap()))
            } else {
                let name = match x.as_str() {
                    "'\\''" => Some("Apostrophe"),
                    "'`'" => Some("Grave"),
                    "'\\\\'" => Some("Backslash"),
                    _ => todo!("literal char {x} {:?}", literal),
                };
                Some(Err(name.map(|n| n.to_string()).unwrap_or(x).into()))
            }
        }
        TokenTree::Punct(ref punct) => {
            Some(Ok(punct.as_char()))
        }
        TokenTree::Ident(ref ident) => {
            let label = ident.span().source_text().unwrap();
            if label.len() == 1 {
                Some(Ok(label.chars().next().unwrap()))
            } else {
                Some(Err(label.into()))
            }
        }
        _ => None,
    }
}

#[cfg(feature = "winit")]
fn get_key_code(tree: TokenTree) -> Option<Ident> {
    match tree {
        TokenTree::Literal(ref literal) => {
            let x = literal.span().source_text().unwrap();
            if x.len() == 1 && x.parse::<u8>().is_ok() {
                eprintln!("got numeric literal {:?}", x);
                Some(Ident::new(&format!("Digit{x}"), Span::call_site()))
            } else {
                let name = match x.as_str() {
                    "'\\''" => Some("Apostrophe"),
                    "'`'" => Some("Grave"),
                    "'\\\\'" => Some("Backslash"),
                    _ => todo!("literal char {x} {:?}", literal),
                };
                name.map(|x| Ident::new(x, Span::call_site()))
            }
        }
        TokenTree::Punct(ref punct) => {
            let name: Option<&str> = match punct.as_char() {
                ';' => Some("Semicolon"),
                ':' => {
                    // TODO: `ctrl-:` Can't be entered on a US ANSI
                    // keyboard only `shift-;` can. Make docs clear this
                    // is the key and not the symbol?

                    // add_shift = true;
                    // Some("Semicolon")
                    Some("Colon")
                }
                ',' => Some("Comma"),
                '.' => Some("Period"),
                '^' => Some("Caret"),
                '=' => Some("Equals"),
                '/' => Some("Slash"),
                '-' => Some("Minus"),
                '*' => Some("Asterisk"),
                '+' => Some("Plus"),
                '@' => Some("At"),
                // _ => None
                _ => todo!("punct {:?}", punct),
            };
            name.map(|n| Ident::new(n, punct.span()))
        }
        TokenTree::Ident(ref ident) => {
            let label = ident.span().source_text().unwrap();
            if label.len() == 1 {
                let name: Option<Cow<'static, str>> = match label.chars().next().unwrap() {
                    // x @ 'A'..='Z' => {
                    x @ 'A'..='Z' => {
                        // I'm not sure I like adding shift.
                        // add_shift = true;
                        // Some(x.to_string().into())
                        Some(format!("Key{x}").into())
                    }
                    x @ 'a'..='z' => {
                        abort!(x, "Use uppercase key names");
                        // let s = x.to_ascii_uppercase().to_string();
                        // Some(s.into())
                    }
                    '_' => Some("Underline".into()),
                    _ => todo!("ident {:?}", ident),
                };
                name.as_ref().map(|n| Ident::new(n, ident.span()))
            } else {
                Some(ident.clone())
            }
        }
        _ => None,
    }
}

#[cfg(not(feature = "winit"))]
fn get_key_code(tree: TokenTree) -> Option<Ident> {
    match tree {
        TokenTree::Literal(ref literal) => {
            let x = literal.span().source_text().unwrap();
            if x.len() == 1 && x.parse::<u8>().is_ok() {
                eprintln!("got numeric literal {:?}", x);
                Some(Ident::new(&format!("Key{x}"), Span::call_site()))
                // Some(Ident::new("Keyx", Span::call_site()))
            } else {
                let name = match x.as_str() {
                    "'\\''" => Some("Apostrophe"),
                    "'`'" => Some("Grave"),
                    "'\\\\'" => Some("Backslash"),
                    _ => todo!("literal char {x} {:?}", literal),
                };
                name.map(|x| Ident::new(x, Span::call_site()))
            }
        }
        TokenTree::Punct(ref punct) => {
            let name: Option<&str> = match punct.as_char() {
                ';' => Some("Semicolon"),
                ':' => {
                    // TODO: `ctrl-:` Can't be entered on a US ANSI
                    // keyboard only `shift-;` can. Make docs clear this
                    // is the key and not the symbol?

                    // add_shift = true;
                    // Some("Semicolon")
                    Some("Colon")
                }
                ',' => Some("Comma"),
                '.' => Some("Period"),
                '^' => Some("Caret"),
                '=' => Some("Equals"),
                '/' => Some("Slash"),
                '-' => Some("Minus"),
                '*' => Some("Asterisk"),
                '+' => Some("Plus"),
                '@' => Some("At"),
                // _ => None
                _ => todo!("punct {:?}", punct),
            };
            name.map(|n| Ident::new(n, punct.span()))
        }
        TokenTree::Ident(ref ident) => {
            let label = ident.span().source_text().unwrap();
            if label.len() == 1 {
                let name: Option<Cow<'static, str>> = match label.chars().next().unwrap() {
                    'A'..='Z' => {
                        Some(label.into())
                    }
                    x @ 'a'..='z' => {
                        abort!(x, "Use uppercase key names");
                        // let s = x.to_ascii_uppercase().to_string();
                        // Some(s.into())
                    }
                    '_' => Some("Underline".into()),
                    _ => todo!("ident {:?}", ident),
                };
                name.as_ref().map(|n| Ident::new(n, ident.span()))
            } else {
                Some(ident.clone())
            }
        }
        _ => None,
    }
}

fn read_modifiers(input: TokenStream) -> (TokenStream, TokenStream) {
    let mut r = TokenStream::new();
    let mut i = input.into_iter().peekable();
    let mut last_tree = None;

    fn is_dash(tree: &TokenTree) -> bool {
        match tree {
            TokenTree::Punct(ref punct) => punct.as_char() == '-',
            _ => false,
        }
    }

    while let Some(tree) = i.next() {
        if i.peek().is_none() || (!is_dash(&tree) && !is_dash(i.peek().unwrap())) {
            last_tree = Some(tree);
            break;
        } else {
            let replacement = match tree {
                TokenTree::Ident(ref ident) => match ident.span().source_text().unwrap().as_str() {
                    "shift" => Some(TokenTree::Group(Group::new(
                        Delimiter::None,
                        modifiers_id(quote! { SHIFT }, Modifier::Shift),
                    ))),
                    "ctrl" => Some(TokenTree::Group(Group::new(
                        Delimiter::None,
                        modifiers_id(quote! { CONTROL }, Modifier::Control),
                    ))),
                    "alt" => Some(TokenTree::Group(Group::new(
                        Delimiter::None,
                        modifiers_id(quote! { ALT }, Modifier::Alt),
                    ))),
                    "super" => Some(TokenTree::Group(Group::new(
                        Delimiter::None,
                        modifiers_id(quote! { SUPER }, Modifier::Super),
                    ))),
                    _ => None,
                },
                TokenTree::Punct(ref punct) => match punct.as_char() {
                    '-' => Some(TokenTree::Punct(Punct::new('|', Spacing::Alone))),
                    _ => None,
                },
                _ => None,
            };
            r.extend([replacement.unwrap_or(tree)]);
        }
    }
    // This will add an empty to finish the expression:
    //
    //    ctrl-alt-EMPTY -> Control | Alt | EMPTY.
    //
    //  And it will provide a valid Modifier when none have been provided.
    r.extend([modifiers_id(quote! { empty() }, Modifier::None)]);
    (
        r,
        TokenStream::from_iter(last_tree.into_iter().chain(i)),
    )
}

fn partial_key_code(input: TokenStream) -> (TokenStream, TokenStream) {
    let (mods, rest) = read_modifiers(input);
    let mut i = rest.into_iter();
    let tree = i.next().expect("No tree");
    let key_code = get_key_code(tree).expect("No key code found");
    let key_code_path = key_code_path(key_code);
    (
        quote! {
            (#mods, #key_code_path)
        },
        TokenStream::from_iter(i),
    )
}

fn partial_key(input: TokenStream) -> (TokenStream, TokenStream) {
    let (mods, rest) = read_modifiers(input);
    let mut i = rest.into_iter();
    let tree = i.next().expect("No tree");
    let key = get_key(tree).expect("No key code found");
    // let key_path = key_path(key);
    (
        quote! {
            (#mods, #key)
        },
        TokenStream::from_iter(i),
    )
}
