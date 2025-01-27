# Modern Yu-Gi-Oh! Draft

[License (MIT)](LICENSE.txt)

A small web app simulating a Hearthstone-style draft using the modern Yu-Gi-Oh! card pool.
This is a personal project to solidify and improve on my skills from university as well as boost my resume/portfolio!

### Building

Until this is deployed, here are the dev build steps:

1. Install [Rust](https://www.rust-lang.org/tools/install) if you don't have it
2. Clone/download the repo through your preferred method
3. Open a terminal and go to into the `backend` folder with it
4. `cargo add` any dependencies you do not have
5. Ensure you don't have any other programs listening on port `localhost:8000`
6. `cargo run` the backend!
7. Leave that terminal running in the background and open a new terminal
8. Use the new terminal to go into the `frontend` folder
9. `cargo add` any missing dependencies and install [Trunk](https://trunkrs.dev/)
10. Ensure you don't have any other programs listening on `localhost:8080`
11. `trunk serve` the frontend! (use `--open` to automatically open the site in your browser)


### Schedule
Number  | Goal      | Timeline      | Notes
--|-----------------|---------------|--------
1 | Learn API       | 9/11 - 9/20   | - lost 2 days to VSCode troubleshooting 💀 <br /> [Github issue here](https://github.com/hadan24/ygo-draft/issues/1)
2 | Display cards   | 9/23 - 10/18  | - lost 3 weeks to sickness 🙃, schedule adjusted <br /> [Github issue here](https://github.com/hadan24/ygo-draft/issues/2)
3 | Finalize API v1 | 10/21 - 10/25 | - change of plans to SPA w/ simple API, schedule adjusted <br /> [Github issue here](https://github.com/hadan24/ygo-draft/issues/12)
4 | Tracing/Errors  | 10/28 - 11/8  | - election stress and other sadness :'), schedule adjusted <br /> [Github issue here](https://github.com/hadan24/ygo-draft/issues/13)
5 | Drafting        | 11/11 - 11/29 | - CSS & needed refactoring delayed to be part of next goal <br /> [Github issue here](https://github.com/hadan24/ygo-draft/issues/3)
6 | Finish Routes   | 12/2 - 12/27  | - lost interest for a bit and had other responsibilities, schedule adjusted <br /> [Github issue here](https://github.com/hadan24/ygo-draft/issues/11)

(12/30/24) Styling and stretch goals will be left for another time. I'm looking to start on more game-related projects.

#### Stretch Goals
Number  | Goal      | Timeline      | Notes
--|-----------------|---------------|--------
1 | Styling         | TBD           | <br /> [Github issue here](https://github.com/hadan24/ygo-draft/issues/16)
2 | Find favicon    | TBD           | <br /> [Github issue here](https://github.com/hadan24/ygo-draft/issues/15)
3 | Pause/Resume    | TBD           | <br /> [Github issue here](https://github.com/hadan24/ygo-draft/issues/14)
4 | Fix DuelingBook | TBD           | <br /> [Github issue here](https://github.com/hadan24/ygo-draft/issues/17)
5 | Deck stats      | TBD           | <br /> [Github issue here](https://github.com/hadan24/ygo-draft/issues/5)
6 | Balance draft   | TBD           | <br /> [Github issue here](https://github.com/hadan24/ygo-draft/issues/6)
7 | Deck exporting  | TBD           | - export to YDK will be included as part of Goal 6 <br /> [Github issue here](https://github.com/hadan24/ygo-draft/issues/7)
8 | DB integration  | TBD           | <br /> [Github issue here](https://github.com/hadan24/ygo-draft/issues/8)
9 | Deploy          | TBD           | <br /> [Github issue here](https://github.com/hadan24/ygo-draft/issues/10)

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