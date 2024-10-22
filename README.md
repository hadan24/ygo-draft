# Modern Yu-Gi-Oh! Draft

[License (MIT)](LICENSE.txt)

A small web app simulating a Hearthstone-style draft using the modern Yu-Gi-Oh! card pool.
This is a personal project to solidify and improve on my skills from university as well as boost my resume/portfolio!

### Schedule
Number  | Goal      | Timeline      | Notes
--|-----------------|---------------|--------
1 | Learn API       | 9/11 - 9/20   | - lost 2 days to VSCode troubleshooting 💀 <br /> [Github issue here](https://github.com/hadan24/ygo-draft/issues/1)
2 | Display cards   | 9/23 - 10/18  | - lost 3 weeks to sickness 🙃, schedule adjusted <br /> [Github issue here](https://github.com/hadan24/ygo-draft/issues/2)
3 | Finalize API v1 | 10/21 - 11/1  | - change of plans to SPA w/ simple API, schedule adjusted
4 | Drafting        | 11/4 - 11/22  | 
5 | Finish Routes   | 11/25 - 12/13 |

#### Stretch Goals
Number  | Goal      | Timeline      | Notes
--|-----------------|---------------|--------
1 | Deck stats      | TBD           |
2 | Balance draft   | TBD           |
3 | Deck exporting  | TBD           | - export to YDK will be included as part of Goal 5
4 | DB integration  | TBD           |

### Resources for Myself
Besides the appropriate docs
- [YGOPRODeck API guide](https://ygoprodeck.com/api-guide/)
- [Rust Cookbook, web APIs](https://rust-lang-nursery.github.io/rust-cookbook/web/clients/apis.html)
- [Axum docs](https://docs.rs/axum/latest/axum/)
- [Joshua Mo, shuttle.rs, Axum guide](https://www.shuttle.rs/blog/2023/12/06/using-axum-rust)
- [Yew docs](https://yew.rs/docs/concepts/function-components)
- [No Boilerplate Yew intro/overview](https://www.youtube.com/watch?v=P4LMfkFLRsI)
- Team Gyata guides [Axum](https://www.gyata.ai/rust/axum) | [Yew](https://www.gyata.ai/rust/yew)
- Brooks Builds tutorials [Axum](https://www.youtube.com/playlist?list=PLrmY5pVcnuE-_CP7XZ_44HN-mDrLQV4nS) | [Yew](https://www.youtube.com/playlist?list=PLrmY5pVcnuE_R5qJ0o30eGw77bWmnrUtL)

### Example Deck Stats
For whenever I get to them:
- Main Deck Monster/Spell/Trap ratio
- Main Deck Monster Attribute ratio
- Main Deck Monster subtype ratio (Tuner/Spirit/etc)
- Extra Deck Monster ratio
- Extra Deck Level/Rank ratio (Synchros/XYZ/Links only)
- Link Monsters w/ bottom markers (warn if >1 Links in deck but 0 w/ bottom markers)
- Highest/lowest ATK/DEF values (among applicable cards)
- Spell/Trap subtype ratio

### Draft Balancing Rules
For whenever I get to them:
- All Main Deck monsters are level 4 or lower.
- All monsters are treated as every type.
- All fusion-summoning effects can summon any Fusion Monster, provided all the materials are correct.
- Ritual and Pendulum Monsters are not included due to their inherent need for extremely high levels of synergy that are practically impossible in draft modes.
- The draft alternates between Main Deck and Extra Deck cards. You'll pick 8 Main Deck cards followed by 3 Extra Deck cards, repeated 5 times.