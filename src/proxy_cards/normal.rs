use crate::atomic_cards::Cardoid;

use super::utils::color_css_class;

// fn normal_card(proxy: &Proxy) -> Option<HtmlProxyCard> {
//     let [card] = &proxy.cardoid.0[..] else {
//         return None;
//     };

//     let colors = color_css_class(card);

//     Some(HtmlProxyCard(link!(
//         r#"<div class="card "#,
//         colors,
//         r#""><div class="title bar"><span class="name">"#,
//         card.name,
//         r#"</span>
//         <span class="cost"><i class="mi mi-r mi-mana mi-shadow"></i></span>
//                 </div>
//                 <img class="art"
//                     src="">
//                 <div class="type-line bar">
//                     <span class="type">Instant</span>
//                 </div>
//                 <div class="text-box sparse">
//                     <p class="rules-text">
//                         Deal 3 damage to any target.
//                     </p>
//                 </div>
//                 <span class="art-credits">goopqj</span>
//             </div>
//             "#
//     )))
// }
