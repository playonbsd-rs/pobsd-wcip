use std::collections::HashMap;

lazy_static! {
    pub static ref GAMETOOS: HashMap<u32, Vec<&'static str>> = {
        let mut gametoos = HashMap::new();

        // The Adventures of Mr. Hat
        gametoos.insert(483693178,vec!["win"]);
        // The Adventures of Shuggy
        gametoos.insert(539464687,vec!["win", "mac", "linux"]);
        // Aeternum
        gametoos.insert(3722587207,vec!["win"]);
        // Airships: Conquer the Skies
        gametoos.insert(1595434339,vec!["win", "mac", "linux"]);
        // Akane the Kunoichi
        gametoos.insert(2214773326,vec!["win"]);
        // Alien Shepherd
        gametoos.insert(2316180984,vec!["win"]);
        // Always Sometimes Monsters
        gametoos.insert(2540995041,vec!["win", "mac", "linux"]);
        // Amazing Princess Sarah
        gametoos.insert(4285628972,vec!["win"]);
        // Amnesia: The Dark Descent
        gametoos.insert(3333072538,vec!["win", "mac", "linux"]);
        // Apocalyptic Vibes
        gametoos.insert(1726424197,vec!["win", "linux"]);
        // Apotheon
        gametoos.insert(1423216523,vec!["win", "mac", "linux"]);
        // Apple Jack 1&2
        gametoos.insert(3789388719,vec!["win"]);
        // Aquaria
        gametoos.insert(1206554311,vec!["win"]);
        // Arx Fatalis
        gametoos.insert(398751234,vec!["win"]);
        // Ashworld
        gametoos.insert(807167811,vec!["win", "mac", "linux"]);
        // Atom Zombie Smasher
        gametoos.insert(3089301381,vec!["win", "mac", "linux"]);
        // The Average Everyday Adventures of Samantha Browne
        gametoos.insert(1635857046,vec!["win", "mac", "linux"]);
        // Axiom Verge
        gametoos.insert(2247085572,vec!["win", "linux"]);
        // Banana Hell
        gametoos.insert(3766266711,vec!["win"]);
        // Barony
        gametoos.insert(3905189042,vec!["win", "mac", "linux"]);
        // Beholder's Lair
        gametoos.insert(2625507310,vec!["win"]);
        // Beneath a Steel Sky
        gametoos.insert(790813605,vec!["win"]);
        // Beyond Sunset
        gametoos.insert(1184128454,vec!["win", "mac"]);
        // Bird Assassin
        gametoos.insert(3634545723,vec!["win"]);
        // Blacken Slash
        gametoos.insert(418673747,vec!["win", "mac", "linux"]);
        // The Blackwell Epiphany
        gametoos.insert(1882725669,vec!["win", "mac", "linux"]);
        // Blastronaut
        gametoos.insert(1023401208,vec!["win", "mac", "linux"]);
        // Bleed
        gametoos.insert(3612462451,vec!["win", "mac", "linux"]);
        // Bleed 2
        gametoos.insert(49516229,vec!["win", "mac", "linux"]);
        // Blossom Tales II: The Minotaur Prince
        gametoos.insert(1228302557,vec!["win", "linux"]);
        // BLUE REVOLVER
        gametoos.insert(3739112806,vec!["win", "mac", "linux"]);
        // Breath of Death VII
        gametoos.insert(156924372,vec!["win"]);
        // Broken Sword II: The Smoking Mirror
        gametoos.insert(2505712031,vec!["win"]);
        // Brotato
        gametoos.insert(4210591076,vec!["win"]);
        // Brushwood Buddies
        gametoos.insert(2277715500,vec!["win", "linux"]);
        // Burning Knight
        gametoos.insert(1773768843,vec!["win", "mac", "linux"]);
        // Caesar 3
        gametoos.insert(3056315765,vec!["win"]);
        // Capsized
        gametoos.insert(323201466,vec!["win", "mac", "linux"]);
        // Capsule Force
        gametoos.insert(465064403,vec!["win", "mac", "linux"]);
        // The Case of the Golden Idol
        gametoos.insert(337072362,vec!["win", "mac"]);
        // Cat Box Paradox
        gametoos.insert(2218927623,vec!["win"]);
        // Cat Cafe Manager
        gametoos.insert(1542931590,vec!["win"]);
        // The Cat Lady
        gametoos.insert(2807805156,vec![]);
        // Celeste
        gametoos.insert(793260538,vec!["win", "mac", "linux"]);
        // Charlie Murder
        gametoos.insert(2198531764,vec!["win", "mac", "linux"]);
        // Chasm
        gametoos.insert(464078685,vec!["win", "mac", "linux"]);
        // Chess Survivors
        gametoos.insert(129588099,vec!["win", "linux"]);
        // CometStriker
        gametoos.insert(3198728447,vec!["win", "mac", "linux"]);
        // Crawlers and Brawlers
        gametoos.insert(2829038399,vec!["win", "mac", "linux"]);
        // CrossCode
        gametoos.insert(3542691004,vec!["win", "mac", "linux"]);
        // Cruelty Squad
        gametoos.insert(666710447,vec!["win"]);
        // Cryptark
        gametoos.insert(3145497575,vec!["win", "mac", "linux"]);
        // Crystal Project
        gametoos.insert(3903369908,vec!["win", "mac", "linux"]);
        // Cthulhu Saves the World
        gametoos.insert(864001885,vec!["win"]);
        // Curse of the Crescent Isle DX
        gametoos.insert(1193243393,vec!["win", "mac", "linux"]);
        // Cursed Gem
        gametoos.insert(682937436,vec!["win", "mac", "linux"]);
        // Cute Cats
        gametoos.insert(428476695,vec!["win"]);
        // Cyber-doge 2077: Meme runner
        gametoos.insert(387531900,vec!["win"]);
        // Dad Quest
        gametoos.insert(4176417594,vec!["win"]);
        // Dark Crypt
        gametoos.insert(2225331499,vec!["win", "mac", "linux"]);
        // Dark Sheep
        gametoos.insert(1425001906,vec!["win", "mac", "linux"]);
        // Dead Cells
        gametoos.insert(200302146,vec!["win", "mac", "linux"]);
        // Dead Pixels
        gametoos.insert(225365062,vec!["win"]);
        // Dead Pixels II
        gametoos.insert(3610527538,vec!["win", "mac", "linux"]);
        // Delta-V: Rings of Saturn
        gametoos.insert(299564569,vec!["win", "mac", "linux"]);
        // Delver
        gametoos.insert(3524978690,vec!["win", "mac", "linux"]);
        // Descent
        gametoos.insert(3246465185,vec!["win", "mac", "linux"]);
        // Descent II
        gametoos.insert(1175074759,vec!["win", "mac", "linux"]);
        // Dice Tribes: Ambitions
        gametoos.insert(3718511298,vec!["win", "mac", "linux"]);
        // Diefrosty
        gametoos.insert(1774769361,vec!["win"]);
        // Diehard Dungeon
        gametoos.insert(2204621548,vec!["win"]);
        // The Dig
        gametoos.insert(1017574798,vec!["win", "mac"]);
        // Dino Eggs: Rebirth
        gametoos.insert(380336525,vec!["win", "mac", "linux"]);
        // The Dishwasher: Vampire Smile
        gametoos.insert(2671587470,vec!["win", "mac", "linux"]);
        // Doki Doki Literature Club!
        gametoos.insert(2078736925,vec!["win", "mac"]);
        // Dome Keeper (formerly Dome Romantik)
        gametoos.insert(4004048401,vec!["win", "mac", "linux"]);
        // Doom
        gametoos.insert(2221465636,vec!["win"]);
        // Doom & Destiny Advanced
        gametoos.insert(2002996976,vec!["win", "mac"]);
        // Downfall
        gametoos.insert(1901504460,vec![]);
        // Draw A Stickman: EPIC
        gametoos.insert(2018199675,vec!["win"]);
        // Dreams in the Witch House
        gametoos.insert(3770453608,vec!["win"]);
        // Droid Assault
        gametoos.insert(3198853854,vec!["win", "mac", "linux"]);
        // Dust: An Elysian Tail
        gametoos.insert(4282888999,vec!["win", "mac", "linux"]);
        // Dustforce DX
        gametoos.insert(2707338748,vec!["win", "mac"]);
        // DwarfCorp
        gametoos.insert(4022765022,vec!["win", "mac", "linux"]);
        // EXAPUNKS
        gametoos.insert(3651630824,vec!["win", "mac", "linux"]);
        // Eagle Island
        gametoos.insert(4097066804,vec!["win", "mac", "linux"]);
        // The Elder Scrolls III: Morrowind
        gametoos.insert(2283104464,vec!["win"]);
        // Eliza
        gametoos.insert(1146203884,vec!["win", "mac", "linux"]);
        // Elliot Quest
        gametoos.insert(720936482,vec!["win"]);
        // Escape Goat
        gametoos.insert(1139036505,vec!["win", "mac", "linux"]);
        // Escape Goat 2
        gametoos.insert(1043056311,vec!["win", "mac", "linux"]);
        // Escape from Monkey Island
        gametoos.insert(4153687982,vec!["win"]);
        // Evoland Legendary Edition
        gametoos.insert(3876099689,vec!["win", "mac", "linux"]);
        // The Excavation of Hob's Barrow
        gametoos.insert(611681995,vec!["win", "mac", "linux"]);
        // Explosionade
        gametoos.insert(3654304404,vec!["win"]);
        // The Expression Amrilato
        gametoos.insert(3323816967,vec!["win", "mac", "linux"]);
        // Fenimore Fillmore: 3 Skulls of the Toltecs
        gametoos.insert(682303823,vec!["win"]);
        // FEZ
        gametoos.insert(2753947488,vec!["win", "mac", "linux"]);
        // Fracture Hell
        gametoos.insert(2301165000,vec!["win"]);
        // FRANZ FURY
        gametoos.insert(2218512780,vec!["win", "mac", "linux"]);
        // Fraymakers
        gametoos.insert(435785947,vec!["win", "mac", "linux"]);
        // Fatal Twelve
        gametoos.insert(3428459646,vec!["win", "mac", "linux"]);
        // Fist Puncher
        gametoos.insert(1345387388,vec!["win", "mac", "linux"]);
        // Flinthook
        gametoos.insert(451366951,vec!["win", "mac", "linux"]);
        // Flotilla
        gametoos.insert(2036469154,vec!["win", "mac", "linux"]);
        // The Forestale
        gametoos.insert(656295889,vec!["win"]);
        // Freespace 2
        gametoos.insert(4018327217,vec!["win"]);
        // Fury's Sky
        gametoos.insert(2173587771,vec!["win"]);
        // Gabriel Knight: Sins of the Fathers
        gametoos.insert(1030920098,vec!["win"]);
        // Game Type
        gametoos.insert(1569704009,vec!["win"]);
        // Gateways
        gametoos.insert(2372419658,vec!["win", "mac", "linux"]);
        // Gemini Rue
        gametoos.insert(3057181722,vec!["win", "mac", "linux"]);
        // Glitchangels
        gametoos.insert(35355806,vec!["win", "mac", "linux"]);
        // A Golden Wake
        gametoos.insert(2178230395,vec!["win", "mac", "linux"]);
        // Goop Loop
        gametoos.insert(133842776,vec!["win", "linux"]);
        // Grand Class Melee 2
        gametoos.insert(4080672227,vec!["win"]);
        // Gravity Ace
        gametoos.insert(1909214212,vec!["win", "linux"]);
        // The Great Plague Exodus
        gametoos.insert(3348305665,vec!["win", "mac", "linux"]);
        // Grimm's Hollow
        gametoos.insert(3971366306,vec!["win"]);
        // Groov
        gametoos.insert(834808254,vec!["win", "mac", "linux"]);
        // Growing Pains
        gametoos.insert(4131645463,vec!["win", "mac", "linux"]);
        // Gunslugs
        gametoos.insert(1339502021,vec!["win", "mac", "linux"]);
        // Gunslugs 3: Rogue Tactics
        gametoos.insert(788014407,vec!["win", "mac", "linux"]);
        // Hack Grid
        gametoos.insert(2911947242,vec!["win", "mac", "linux"]);
        // HackNet
        gametoos.insert(1723734015,vec!["win", "mac", "linux"]);
        // Haiki
        gametoos.insert(1922468029,vec!["win", "mac", "linux"]);
        // Hands of Necromancy
        gametoos.insert(2112503295,vec!["win", "mac", "linux"]);
        // Hashi: Light
        gametoos.insert(1381809646,vec!["win", "mac"]);
        // HeXen: Beyond Heretic
        gametoos.insert(2710795324,vec!["win"]);
        // HeXen II
        gametoos.insert(1292989339,vec!["win"]);
        // Hedon
        gametoos.insert(2718236267,vec!["win", "linux"]);
        // Heretic: Shadow of the Serpent Riders
        gametoos.insert(4017927205,vec!["win"]);
        // Heroes of Loot
        gametoos.insert(3282404328,vec!["win", "mac", "linux"]);
        // Heroine's Quest: The Herald of Ragnarok
        gametoos.insert(2046289092,vec!["win", "linux"]);
        // Highway Blossoms
        gametoos.insert(1733122226,vec!["win", "mac", "linux"]);
        // Hive
        gametoos.insert(1945992277,vec!["win", "mac", "linux"]);
        // Hiveswap Friendsim
        gametoos.insert(3552947413,vec!["win", "mac", "linux"]);
        // Human Diaspora
        gametoos.insert(121750763,vec!["win", "linux"]);
        // Hyphen
        gametoos.insert(324093715,vec!["win", "linux"]);
        // I am Sakuya: Touhou FPS Game
        gametoos.insert(3469999166,vec!["win"]);
        // I Have No Mouth And I Must Scream
        gametoos.insert(3806089600,vec!["win", "mac", "linux"]);
        // I MAED A GAM3 W1TH Z0MB1ES 1N IT!!!1
        gametoos.insert(416837008,vec!["win", "linux"]);
        // Ib
        gametoos.insert(1337362582,vec!["win"]);
        // Indiana Jones and the Fate of Atlantis
        gametoos.insert(358600547,vec!["win", "mac"]);
        // Indirection
        gametoos.insert(2210961862,vec!["win", "mac", "linux"]);
        // In the House of Silence
        gametoos.insert(362835569,vec!["win", "mac", "linux"]);
        // Ion Fury
        gametoos.insert(3870767401,vec!["win", "linux"]);
        // Jagged Alliance 2
        gametoos.insert(1817723360,vec!["win"]);
        // Jon Shafer's At the Gates
        gametoos.insert(1880284321,vec!["win", "mac", "linux"]);
        // Joyspring
        gametoos.insert(1156312590,vec!["win", "mac", "linux"]);
        // Kathy Rain
        gametoos.insert(2332019830,vec!["win", "mac"]);
        // Kitsune Zero
        gametoos.insert(1291258200,vec!["win", "mac", "linux"]);
        // Kitty Tactics
        gametoos.insert(1467566601,vec!["win", "mac", "linux"]);
        // Kotel Ne Gori: A Friend of Lena Boots
        gametoos.insert(1642070196,vec!["win", "linux"]);
        // Lamplight City
        gametoos.insert(2676405412,vec!["win", "mac", "linux"]);
        // LaserCat
        gametoos.insert(3647199328,vec!["win"]);
        // Learn Japanese To Survive! Hiragana Battle
        gametoos.insert(203113299,vec!["win", "mac"]);
        // Lila's Sky Ark
        gametoos.insert(875907547,vec!["win", "mac", "linux"]);
        // Lingo
        gametoos.insert(1449537939,vec!["win"]);
        // Little Racers STREET
        gametoos.insert(2011788384,vec!["win", "mac", "linux"]);
        // Long Live the Queen
        gametoos.insert(1507863460,vec!["win", "mac", "linux"]);
        // The Longest Journey
        gametoos.insert(2559734008,vec!["win"]);
        // Loom
        gametoos.insert(2592636700,vec!["win", "mac"]);
        // Lost Cities
        gametoos.insert(2843146844,vec!["win", "mac", "linux"]);
        // Luck be a Landlord
        gametoos.insert(1295220560,vec!["win", "mac", "linux"]);
        // Lycanthorn II
        gametoos.insert(567957947,vec!["win", "linux"]);
        // Malice & Greed
        gametoos.insert(258323205,vec!["win"]);
        // Mega Man Legacy Collection
        gametoos.insert(51190182,vec!["win"]);
        // Meganoid
        gametoos.insert(1825370595,vec!["win", "mac", "linux"]);
        // Mercenary Kings
        gametoos.insert(3571620251,vec![]);
        // Mewnbase
        gametoos.insert(109730221,vec!["win", "linux"]);
        // Miasma: Citizens of Free Thought
        gametoos.insert(2672134974,vec!["win", "mac", "linux"]);
        // Miasma 2: Freedom Uprising
        gametoos.insert(1528724953,vec!["win", "mac", "linux"]);
        // Micro Mages
        gametoos.insert(2796295634,vec!["win"]);
        // MidBoss
        gametoos.insert(3459136742,vec!["win", "mac", "linux"]);
        // Mirrorama
        gametoos.insert(1530248213,vec!["win", "mac", "linux"]);
        // Mobius Front 83
        gametoos.insert(21084566,vec!["win", "mac", "linux"]);
        // Molek-Syntez
        gametoos.insert(1704722141,vec!["win", "mac", "linux"]);
        // Monster Outbreak
        gametoos.insert(1388189550,vec!["win"]);
        // Moonshot - The Great Espionage
        gametoos.insert(3556045612,vec!["win", "linux"]);
        // Mount Your Friends
        gametoos.insert(2365126988,vec![]);
        // Mud River
        gametoos.insert(3479442855,vec!["win", "mac", "linux"]);
        // Murder Miners
        gametoos.insert(2056930988,vec!["win", "mac", "linux"]);
        // Myst
        gametoos.insert(670769280,vec!["win", "mac"]);
        // Myst III: Exile
        gametoos.insert(2246386912,vec!["win", "mac"]);
        // Neofeud
        gametoos.insert(1640898753,vec!["win"]);
        // NeuroVoider
        gametoos.insert(1353993109,vec!["win", "mac", "linux"]);
        // Nightfall Hacker
        gametoos.insert(3958280650,vec!["win", "mac", "linux"]);
        // Northgard
        gametoos.insert(673713456,vec!["win", "mac", "linux"]);
        // Nuclear Blaze
        gametoos.insert(2481842725,vec!["win", "linux"]);
        // One Finger Death Punch
        gametoos.insert(167113526,vec!["win"]);
        // Open Sorcery
        gametoos.insert(1719301452,vec!["win", "mac", "linux"]);
        // Opus Magnum
        gametoos.insert(1636371499,vec!["win", "mac", "linux"]);
        // Overdriven Reloaded
        gametoos.insert(602065011,vec!["win", "mac", "linux"]);
        // Owlboy
        gametoos.insert(3802042723,vec!["win", "mac", "linux"]);
        // Paladin
        gametoos.insert(1798006366,vec!["win", "mac", "linux"]);
        // Paralyzed
        gametoos.insert(1797817451,vec!["win", "mac", "linux"]);
        // Penny Arcade's On the Rain-Slick Precipice of Darkness 3
        gametoos.insert(1442934177,vec!["win"]);
        // Penny Arcade's On the Rain-Slick Precipice of Darkness 4
        gametoos.insert(1442683914,vec!["win"]);
        // Phoenix Force
        gametoos.insert(2948546407,vec!["win"]);
        // Postal
        gametoos.insert(1975193614,vec![]);
        // Press X to Not Die
        gametoos.insert(2067119678,vec!["win", "mac", "linux"]);
        // Primal Light
        gametoos.insert(3770804040,vec!["win", "mac", "linux"]);
        // Primordia
        gametoos.insert(3919727532,vec!["win", "mac", "linux"]);
        // Project Crypt
        gametoos.insert(3289329542,vec!["win"]);
        // Project Heartbeat
        gametoos.insert(3936955816,vec!["win", "linux"]);
        // Quake
        gametoos.insert(2183659576,vec![]);
        // Quake II
        gametoos.insert(1711967880,vec!["win"]);
        // Quake III Arena
        gametoos.insert(3398758898,vec!["win"]);
        // Quest for Infamy
        gametoos.insert(318839218,vec!["win", "linux"]);
        // Quest of Graal
        gametoos.insert(3043627757,vec!["win", "mac", "linux"]);
        // Retro City Rampage DX
        gametoos.insert(193479195,vec!["win", "mac", "linux"]);
        // Return to Castle Wolfenstein
        gametoos.insert(2159459980,vec!["win"]);
        // Rex Rocket
        gametoos.insert(1217074036,vec!["win", "mac", "linux"]);
        // Rise to Ruins
        gametoos.insert(3396616260,vec!["win", "mac", "linux"]);
        // Riven
        gametoos.insert(228019385,vec!["win", "mac"]);
        // Rodent and Plank: Secret Origin
        gametoos.insert(759117852,vec!["win", "linux"]);
        // Rogue Legacy
        gametoos.insert(2217939096,vec!["win", "mac", "linux"]);
        // Rogue State Revolution
        gametoos.insert(3284984257,vec!["win", "linux"]);
        // RollerCoaster Tycoon
        gametoos.insert(2239837498,vec!["win"]);
        // RollerCoaster Tycoon 2
        gametoos.insert(366494756,vec!["win"]);
        // Roma Invicta
        gametoos.insert(2601652100,vec!["win", "mac", "linux"]);
        // Ruggnar
        gametoos.insert(738681263,vec!["win"]);
        // ROTA
        gametoos.insert(2122792745,vec!["win", "mac", "linux"]);
        // RUN: The world in-between
        gametoos.insert(3069854191,vec!["win"]);
        // RuneScape
        gametoos.insert(779289133,vec!["win", "mac"]);
        // Rushberry Mercs
        gametoos.insert(3993039567,vec!["win"]);
        // SEGA Mega Drive and Genesis Classics
        gametoos.insert(1828657701,vec![]);
        // SJ-19 Learns To Love!
        gametoos.insert(1658453052,vec!["win", "linux"]);
        // Sokobos
        gametoos.insert(2164501758,vec!["win", "mac", "linux"]);
        // SokoChess
        gametoos.insert(2074523085,vec!["win", "mac", "linux"]);
        // SokoChess White
        gametoos.insert(146962511,vec!["win", "mac", "linux"]);
        // SUMICO - The Numbers Game
        gametoos.insert(2747645675,vec!["win", "mac", "linux"]);
        // Salt and Sanctuary
        gametoos.insert(2627238074,vec![]);
        // Sam & Max Hit the Road
        gametoos.insert(722522936,vec!["win", "mac"]);
        // satryn deluxe
        gametoos.insert(3027209299,vec!["win", "mac", "linux"]);
        // Session Seven
        gametoos.insert(586137124,vec!["win", "linux"]);
        // Seven Kingdoms: Ancient Adversaries
        gametoos.insert(1470641001,vec!["win"]);
        // Shadow Warrior Classic Redux
        gametoos.insert(2547716035,vec!["win", "mac"]);
        // Shigatari
        gametoos.insert(4010254425,vec!["win", "mac", "linux"]);
        // Shipwreck
        gametoos.insert(2125892255,vec!["win", "linux"]);
        // The Shivah
        gametoos.insert(3733177703,vec!["win", "mac", "linux"]);
        // Shrine
        gametoos.insert(1238697931,vec!["win", "linux"]);
        // Shrine II
        gametoos.insert(449255765,vec!["win", "linux"]);
        // Signs of Life
        gametoos.insert(4017687802,vec!["win"]);
        // Simona's Requiem
        gametoos.insert(4067415938,vec!["win", "mac", "linux"]);
        // Sir Questionnaire
        gametoos.insert(865905578,vec!["win", "linux"]);
        // Skulls of the Shogun
        gametoos.insert(672563997,vec!["win", "mac", "linux"]);
        // Slay the Spire
        gametoos.insert(3776946340,vec!["win", "mac", "linux"]);
        // Snake Core
        gametoos.insert(1227673738,vec!["win", "linux"]);
        // Solar Rogue
        gametoos.insert(3586711541,vec!["win", "linux"]);
        // Solaroids: Prologue
        gametoos.insert(3713635070,vec!["win", "mac", "linux"]);
        // Soulcaster I & II
        gametoos.insert(4065223331,vec!["win", "mac", "linux"]);
        // SpaceChem
        gametoos.insert(940030390,vec!["win", "mac", "linux"]);
        // Space Grunts
        gametoos.insert(2898300560,vec!["win", "mac", "linux"]);
        // Space Grunts 2
        gametoos.insert(4206395974,vec!["win", "mac", "linux"]);
        // Space Haven
        gametoos.insert(2607736985,vec!["win", "mac", "linux"]);
        // Spaceport Hope
        gametoos.insert(2457286670,vec!["win"]);
        // SpeedRunners
        gametoos.insert(2860433649,vec!["win", "mac", "linux"]);
        // Spellbook Demonslayers
        gametoos.insert(174433159,vec!["win"]);
        // Star Wars: Jedi Knight - Jedi Academy
        gametoos.insert(1026195232,vec!["win", "mac"]);
        // Stardew Valley
        gametoos.insert(2245791810,vec!["win", "mac", "linux"]);
        // StarPrey
        gametoos.insert(2954122711,vec!["win", "mac", "linux"]);
        // Star-Twine
        gametoos.insert(4187681019,vec!["win", "mac", "linux"]);
        // Steel Assault
        gametoos.insert(3234895239,vec!["win", "mac", "linux"]);
        // Strangeland
        gametoos.insert(2843820624,vec!["win", "mac", "linux"]);
        // Strife: Veteran Edition
        gametoos.insert(1108721645,vec!["win", "mac", "linux"]);
        // Suits: A Business RPG
        gametoos.insert(1026879291,vec!["win", "mac", "linux"]);
        // Sunrider: Mask of Arcadius
        gametoos.insert(2672676346,vec!["win", "mac", "linux"]);
        // Super Amazing Wagon Adventure
        gametoos.insert(8820847,vec!["win"]);
        // Super Bernie World
        gametoos.insert(3323450609,vec!["win", "mac", "linux"]);
        // Super Blood Hockey
        gametoos.insert(1848813103,vec!["win", "mac", "linux"]);
        // Super Rad Raygun
        gametoos.insert(3579077245,vec!["win", "mac", "linux"]);
        // Sword of the Stars: The Pit
        gametoos.insert(1892619583,vec!["win", "mac", "linux"]);
        // Tales of Maj'Eyal
        gametoos.insert(3286093679,vec!["win", "mac", "linux"]);
        // Tanglewood
        gametoos.insert(3559422909,vec!["win", "mac", "linux"]);
        // TD Worlds
        gametoos.insert(878117955,vec!["win"]);
        // Technobabylon
        gametoos.insert(2092934091,vec!["win", "mac", "linux"]);
        // Terraria
        gametoos.insert(717044249,vec!["win", "mac", "linux"]);
        // Thukothea Defender
        gametoos.insert(1547648105,vec!["win", "linux"]);
        // Timespinner
        gametoos.insert(1384576505,vec!["win", "mac", "linux"]);
        // Time's Up in Tiny Town
        gametoos.insert(1405076312,vec!["win", "mac", "linux"]);
        // Titan Attacks!
        gametoos.insert(664400785,vec!["win", "mac", "linux"]);
        // Toonstruck
        gametoos.insert(1234201391,vec!["win", "mac", "linux"]);
        // TowerFall: Ascension
        gametoos.insert(3373980756,vec!["win", "mac", "linux"]);
        // Ultra Hat Dimension
        gametoos.insert(3756401890,vec!["win", "mac", "linux"]);
        // Ultratron
        gametoos.insert(102280636,vec!["win", "mac", "linux"]);
        // Unavowed
        gametoos.insert(2953591878,vec!["win", "mac", "linux"]);
        // Underlings
        gametoos.insert(4231456798,vec!["win", "mac", "linux"]);
        // Unexplored
        gametoos.insert(3047124949,vec!["win", "mac", "linux"]);
        // Unholy Heights
        gametoos.insert(277921868,vec!["win"]);
        // Urtuk: The Desolation
        gametoos.insert(2762679391,vec!["win", "mac", "linux"]);
        // Virtual Cottage
        gametoos.insert(2574776493,vec!["win", "mac", "linux"]);
        // A Virus Named Tom
        gametoos.insert(1171843366,vec!["win", "mac", "linux"]);
        // Voidrun
        gametoos.insert(3471429174,vec!["win"]);
        // Vomitoreum
        gametoos.insert(2781419886,vec!["win", "linux"]);
        // Vylan
        gametoos.insert(1361928155,vec!["win", "linux"]);
        // Weapon of Choice
        gametoos.insert(3735222255,vec!["win"]);
        // Whispers of a Machine
        gametoos.insert(3859508815,vec!["win", "mac"]);
        // Wish Project
        gametoos.insert(3764779777,vec!["win", "linux"]);
        // WizOrb
        gametoos.insert(730050274,vec!["win", "mac", "linux"]);
        // Woodland Empire
        gametoos.insert(1757161184,vec!["win", "linux"]);
        // Word Game: Episode 0
        gametoos.insert(3423986932,vec!["win", "mac"]);
        // Wrath: Aeon of Ruin
        gametoos.insert(2399402283,vec!["win"]);
        // Wyv and Keep
        gametoos.insert(4148782727,vec!["win", "mac", "linux"]);
        // X-Com: UFO Defense (aka UFO: Enemy Unknown)
        gametoos.insert(1173623993,vec!["win"]);
        // Yume Nikki
        gametoos.insert(3619239825,vec!["win"]);
        // The Zachtronics Solitaire Collection
        gametoos.insert(4128686703,vec!["win", "mac", "linux"]);
        gametoos
    };
}
