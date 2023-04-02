use std::collections::HashMap;

lazy_static! {
    pub static ref GAMETOOS: HashMap<u32, Vec<&'static str>> = {
        let mut gametoos = HashMap::new();

        // AaaaaAAaaaAAAaaAAAAaAAAAA!!! for the Awesome
        gametoos.insert(3983481694,vec!["linux", "win", "mac"]);
        // The Adventures of Mr. Hat
        gametoos.insert(483693178,vec!["win"]);
        // The Adventures of Shuggy
        gametoos.insert(539464687,vec!["linux", "win", "mac"]);
        // Aedemphia
        gametoos.insert(2086575957,vec![]);
        // Aeternum
        gametoos.insert(3722587207,vec!["win"]);
        // Airships: Conquer the Skies
        gametoos.insert(1595434339,vec!["linux", "win", "mac"]);
        // Akane the Kunoichi
        gametoos.insert(2214773326,vec!["win", "ios"]);
        // Alien Shepherd
        gametoos.insert(2316180984,vec![]);
        // Always Sometimes Monsters
        gametoos.insert(2540995041,vec!["linux", "win", "mac", "android", "ios"]);
        // Amazing Princess Sarah
        gametoos.insert(4285628972,vec!["win"]);
        // Amnesia: The Dark Descent
        gametoos.insert(3333072538,vec!["linux", "win", "mac"]);
        // Apocalyptic Vibes
        gametoos.insert(1726424197,vec!["win"]);
        // Apotheon
        gametoos.insert(1423216523,vec!["linux", "win", "mac"]);
        // Apple Jack 1&2
        gametoos.insert(3789388719,vec!["win"]);
        // Aquaria
        gametoos.insert(1206554311,vec!["linux", "win", "mac", "ios"]);
        // Arx Fatalis
        gametoos.insert(398751234,vec!["win"]);
        // Ashworld
        gametoos.insert(807167811,vec!["linux", "win", "mac", "ios"]);
        // Atom Zombie Smasher
        gametoos.insert(3089301381,vec!["linux", "win", "mac"]);
        // The Average Everyday Adventures of Samantha Browne
        gametoos.insert(1635857046,vec!["linux", "win", "mac"]);
        // Axiom Verge
        gametoos.insert(2247085572,vec!["linux", "win", "mac"]);
        // Baldur's Gate
        gametoos.insert(3562870270,vec!["linux", "win", "mac", "ios"]);
        // Baldur's Gate 2
        gametoos.insert(2734409576,vec!["win", "mac"]);
        // Banana Hell
        gametoos.insert(3766266711,vec!["win"]);
        // Barony
        gametoos.insert(3905189042,vec!["linux", "win", "mac"]);
        // Beholder's Lair
        gametoos.insert(2625507310,vec!["win"]);
        // Beneath a Steel Sky
        gametoos.insert(790813605,vec!["linux", "win", "dos", "mac", "ios"]);
        // Beyond Sunset
        gametoos.insert(1184128454,vec!["win"]);
        // Bird Assassin
        gametoos.insert(3634545723,vec!["win"]);
        // Blacken Slash
        gametoos.insert(418673747,vec!["linux", "win", "mac"]);
        // The Blackwell Epiphany
        gametoos.insert(1882725669,vec!["linux", "win", "mac"]);
        // Blastronaut
        gametoos.insert(1023401208,vec!["linux", "win"]);
        // Bleed
        gametoos.insert(3612462451,vec!["linux", "win", "mac"]);
        // Bleed 2
        gametoos.insert(49516229,vec!["linux", "win", "mac"]);
        // Blossom Tales II: The Minotaur Prince
        gametoos.insert(1228302557,vec!["win"]);
        // BLUE REVOLVER
        gametoos.insert(3739112806,vec!["linux", "win", "mac"]);
        // Bottomless
        gametoos.insert(4044800749,vec![]);
        // Breath of Death VII
        gametoos.insert(156924372,vec!["win"]);
        // Broken Sword II: The Smoking Mirror
        gametoos.insert(2505712031,vec!["win"]);
        // Broken Sword: The Shadow of the Templars
        gametoos.insert(955315214,vec![]);
        // Brotato
        gametoos.insert(4210591076,vec!["win"]);
        // Brushwood Buddies
        gametoos.insert(2277715500,vec!["linux", "win"]);
        // Burning Knight
        gametoos.insert(1773768843,vec!["linux", "win", "mac"]);
        // Caesar 3
        gametoos.insert(3056315765,vec!["win"]);
        // Cannon Fodder
        gametoos.insert(3776853542,vec!["linux", "win", "dos", "mac", "blackberry"]);
        // Cannon Fodder 2
        gametoos.insert(1336782864,vec!["dos"]);
        // Capsized
        gametoos.insert(323201466,vec!["linux", "win", "mac"]);
        // Capsule Force
        gametoos.insert(465064403,vec!["linux", "win", "mac"]);
        // Capsule Wars
        gametoos.insert(1970977409,vec![]);
        // The Case of the Golden Idol
        gametoos.insert(337072362,vec!["win", "mac"]);
        // Cat Box Paradox
        gametoos.insert(2218927623,vec!["win"]);
        // Cat Cafe Manager
        gametoos.insert(1542931590,vec!["win"]);
        // The Cat Lady
        gametoos.insert(2807805156,vec!["linux", "win"]);
        // Cave - Legend of the Forge
        gametoos.insert(815019299,vec!["linux", "win", "mac"]);
        // Celeste
        gametoos.insert(793260538,vec!["linux", "win", "mac"]);
        // Chaos Heart
        gametoos.insert(1904779203,vec![]);
        // Charlie Murder
        gametoos.insert(2198531764,vec!["linux", "win", "mac"]);
        // Chasm
        gametoos.insert(464078685,vec!["linux", "win", "mac"]);
        // Chess Survivors
        gametoos.insert(129588099,vec!["win"]);
        // CometStriker
        gametoos.insert(3198728447,vec!["linux", "win", "mac"]);
        // Command & Conquer
        gametoos.insert(1935386369,vec!["win", "dos", "mac"]);
        // Command & Conquer: Red Alert
        gametoos.insert(142214884,vec!["win", "dos"]);
        // Crawlers and Brawlers
        gametoos.insert(2829038399,vec!["linux", "win", "mac"]);
        // CrossCode
        gametoos.insert(3542691004,vec!["linux", "win", "mac"]);
        // Cruelty Squad
        gametoos.insert(666710447,vec!["win"]);
        // Cryptark
        gametoos.insert(3145497575,vec!["linux", "win", "mac"]);
        // Crystal Project
        gametoos.insert(3903369908,vec!["linux", "win", "mac"]);
        // Cthulhu Saves the World
        gametoos.insert(864001885,vec!["win"]);
        // Curse of the Crescent Isle DX
        gametoos.insert(1193243393,vec!["linux", "win", "mac"]);
        // Cursed Gem
        gametoos.insert(682937436,vec!["linux", "win", "mac"]);
        // Cute Cats
        gametoos.insert(428476695,vec!["win"]);
        // Cyber-doge 2077: Meme runner
        gametoos.insert(387531900,vec!["win"]);
        // Dad Quest
        gametoos.insert(4176417594,vec!["win"]);
        // Dark Crypt
        gametoos.insert(2225331499,vec!["linux", "win", "mac"]);
        // Dark Sheep
        gametoos.insert(1425001906,vec!["linux", "win", "mac"]);
        // Dead Cells
        gametoos.insert(200302146,vec!["linux", "win", "mac", "android", "ios"]);
        // Dead Pixels
        gametoos.insert(225365062,vec!["win"]);
        // Dead Pixels II
        gametoos.insert(3610527538,vec![]);
        // Delta-V: Rings of Saturn
        gametoos.insert(299564569,vec!["win"]);
        // Delver
        gametoos.insert(3524978690,vec!["linux", "win", "mac", "android"]);
        // Democracy 3
        gametoos.insert(2585438391,vec!["linux", "win", "mac", "ios"]);
        // Descent
        gametoos.insert(3246465185,vec!["win", "dos", "mac"]);
        // Descent II
        gametoos.insert(1175074759,vec!["win", "dos", "mac"]);
        // Diablo
        gametoos.insert(3982093056,vec!["win", "mac"]);
        // Dice Tribes: Ambitions
        gametoos.insert(3718511298,vec!["win"]);
        // Diefrosty
        gametoos.insert(1774769361,vec!["win"]);
        // Diehard Dungeon
        gametoos.insert(2204621548,vec!["win"]);
        // The Dig
        gametoos.insert(1017574798,vec!["linux", "win", "dos", "mac"]);
        // Dino Eggs: Rebirth
        gametoos.insert(380336525,vec!["linux", "win", "mac"]);
        // The Dishwasher: Vampire Smile
        gametoos.insert(2671587470,vec!["linux", "win", "mac"]);
        // Doki Doki Literature Club!
        gametoos.insert(2078736925,vec!["linux", "win", "mac"]);
        // Dome Keeper (formerly Dome Romantik)
        gametoos.insert(4004048401,vec!["linux", "win", "mac"]);
        // Doom
        gametoos.insert(2221465636,vec!["linux", "win", "dos", "mac", "ios"]);
        // Doom & Destiny Advanced
        gametoos.insert(2002996976,vec!["linux", "win", "mac", "android", "ios", "winphone"]);
        // Doppelganger
        gametoos.insert(331457351,vec![]);
        // Downfall
        gametoos.insert(1901504460,vec!["win"]);
        // Draw A Stickman: EPIC
        gametoos.insert(2018199675,vec!["win", "mac", "android", "ios", "winphone"]);
        // Dreams in the Witch House
        gametoos.insert(3770453608,vec!["win"]);
        // Droid Assault
        gametoos.insert(3198853854,vec!["linux", "win", "mac"]);
        // Dune 2000
        gametoos.insert(611242023,vec!["win"]);
        // Dust: An Elysian Tail
        gametoos.insert(4282888999,vec!["linux", "win", "mac", "ios"]);
        // Dustforce DX
        gametoos.insert(2707338748,vec!["linux", "win", "mac"]);
        // DwarfCorp
        gametoos.insert(4022765022,vec!["linux", "win", "mac"]);
        // EXAPUNKS
        gametoos.insert(3651630824,vec!["linux", "win", "mac"]);
        // Eagle Island
        gametoos.insert(4097066804,vec!["win"]);
        // The Elder Scrolls III: Morrowind
        gametoos.insert(2283104464,vec!["win"]);
        // Eliza
        gametoos.insert(1146203884,vec!["linux", "win", "mac"]);
        // Elliot Quest
        gametoos.insert(720936482,vec!["linux", "win"]);
        // Escape Goat
        gametoos.insert(1139036505,vec!["linux", "win", "mac"]);
        // Escape Goat 2
        gametoos.insert(1043056311,vec!["linux", "win", "mac"]);
        // Escape from Monkey Island
        gametoos.insert(4153687982,vec!["win", "mac"]);
        // Evoland Legendary Edition
        gametoos.insert(3876099689,vec!["linux", "win", "mac"]);
        // The Excavation of Hob's Barrow
        gametoos.insert(611681995,vec!["linux", "win", "mac"]);
        // Explosionade
        gametoos.insert(3654304404,vec!["win"]);
        // The Expression Amrilato
        gametoos.insert(3323816967,vec!["linux", "win", "mac", "android", "ios"]);
        // Fenimore Fillmore: 3 Skulls of the Toltecs
        gametoos.insert(682303823,vec!["win"]);
        // FEZ
        gametoos.insert(2753947488,vec!["linux", "win", "mac", "ios"]);
        // Fracture Hell
        gametoos.insert(2301165000,vec![]);
        // FRANZ FURY
        gametoos.insert(2218512780,vec![]);
        // Fraymakers
        gametoos.insert(435785947,vec!["win"]);
        // FTL
        gametoos.insert(97494589,vec!["linux", "win", "mac", "ios"]);
        // Fatal Twelve
        gametoos.insert(3428459646,vec!["linux", "win", "mac"]);
        // Fist Puncher
        gametoos.insert(1345387388,vec!["linux", "win", "mac"]);
        // Flinthook
        gametoos.insert(451366951,vec!["linux", "win", "mac"]);
        // Flotilla
        gametoos.insert(2036469154,vec!["linux", "win", "mac"]);
        // Forest's Secret: Mystery of the Frost
        gametoos.insert(3274931779,vec!["linux", "win", "mac"]);
        // The Forestale
        gametoos.insert(656295889,vec!["win"]);
        // Freespace 2
        gametoos.insert(4018327217,vec!["win"]);
        // Fury's Sky
        gametoos.insert(2173587771,vec!["win"]);
        // Gabriel Knight: Sins of the Fathers
        gametoos.insert(1030920098,vec!["win", "dos", "mac"]);
        // Game Type
        gametoos.insert(1569704009,vec!["win"]);
        // Gateways
        gametoos.insert(2372419658,vec!["linux", "win", "mac"]);
        // Gemini Rue
        gametoos.insert(3057181722,vec!["linux", "win", "mac"]);
        // Glitchangels
        gametoos.insert(35355806,vec![]);
        // A Golden Wake
        gametoos.insert(2178230395,vec!["linux", "win", "mac"]);
        // Goop Loop
        gametoos.insert(133842776,vec!["linux", "win"]);
        // Grand Class Melee 2
        gametoos.insert(4080672227,vec!["win"]);
        // Gravity Ace
        gametoos.insert(1909214212,vec!["linux", "win"]);
        // The Great Adventurer
        gametoos.insert(2489317563,vec![]);
        // The Great Plague Exodus
        gametoos.insert(3348305665,vec!["win"]);
        // Grimm's Hollow
        gametoos.insert(3971366306,vec!["win"]);
        // Groov
        gametoos.insert(834808254,vec!["linux", "win", "mac"]);
        // Groundskeeper 2
        gametoos.insert(1018972463,vec!["win", "android", "ios"]);
        // Growing Pains
        gametoos.insert(4131645463,vec!["linux", "win", "mac"]);
        // Gunslugs
        gametoos.insert(1339502021,vec!["linux", "win", "mac", "ios"]);
        // Gunslugs 3: Rogue Tactics
        gametoos.insert(788014407,vec![]);
        // Hack Grid
        gametoos.insert(2911947242,vec!["linux", "win", "mac"]);
        // HackNet
        gametoos.insert(1723734015,vec!["linux", "win", "mac"]);
        // Haiki
        gametoos.insert(1922468029,vec!["linux", "win", "mac"]);
        // Hallow's Eve
        gametoos.insert(3601364004,vec![]);
        // Hands of Necromancy
        gametoos.insert(2112503295,vec!["linux", "win", "mac"]);
        // Hashi: Light
        gametoos.insert(1381809646,vec![]);
        // HeXen: Beyond Heretic
        gametoos.insert(2710795324,vec!["win", "dos", "mac"]);
        // HeXen II
        gametoos.insert(1292989339,vec!["win"]);
        // Hedon
        gametoos.insert(2718236267,vec!["win"]);
        // Heretic: Shadow of the Serpent Riders
        gametoos.insert(4017927205,vec!["win", "dos"]);
        // Heroes of Loot
        gametoos.insert(3282404328,vec!["linux", "win", "mac", "android", "ios"]);
        // Heroes of Might and Magic 2
        gametoos.insert(2751323193,vec!["win"]);
        // Heroine's Quest: The Herald of Ragnarok
        gametoos.insert(2046289092,vec!["linux", "win"]);
        // Highway Blossoms
        gametoos.insert(1733122226,vec!["linux", "win", "mac"]);
        // Hive
        gametoos.insert(1945992277,vec!["win", "dos"]);
        // Hiveswap Friendsim
        gametoos.insert(3552947413,vec!["linux", "win", "mac"]);
        // Human Diaspora
        gametoos.insert(121750763,vec!["linux", "win"]);
        // Hyphen
        gametoos.insert(324093715,vec!["linux", "win"]);
        // I am Sakuya: Touhou FPS Game
        gametoos.insert(3469999166,vec!["win"]);
        // I Have No Mouth And I Must Scream
        gametoos.insert(3806089600,vec!["linux", "win", "dos", "mac", "android", "ios"]);
        // I MAED A GAM3 W1TH Z0MB1ES 1N IT!!!1
        gametoos.insert(416837008,vec!["linux", "win"]);
        // Ib
        gametoos.insert(1337362582,vec!["win"]);
        // Icewind Dale
        gametoos.insert(3337798066,vec!["win", "mac", "ios"]);
        // INC: The Beginning
        gametoos.insert(368879529,vec![]);
        // Indiana Jones and the Fate of Atlantis
        gametoos.insert(358600547,vec!["linux", "win", "dos", "mac"]);
        // Indirection
        gametoos.insert(2210961862,vec!["linux", "win", "mac"]);
        // In the House of Silence
        gametoos.insert(362835569,vec!["linux", "win", "mac"]);
        // Ion Fury
        gametoos.insert(3870767401,vec!["linux", "win"]);
        // Jagged Alliance 2
        gametoos.insert(1817723360,vec!["dos"]);
        // Jam and the Mystery of the Mysteriously Spooky Mansion
        gametoos.insert(3111022695,vec!["linux", "win", "mac"]);
        // Jazz Jackrabbit
        gametoos.insert(4030032499,vec!["dos"]);
        // Jon Shafer's At the Gates
        gametoos.insert(1880284321,vec!["linux", "win", "mac", "ios"]);
        // Joyspring
        gametoos.insert(1156312590,vec!["win"]);
        // Kathy Rain
        gametoos.insert(2332019830,vec!["win", "mac", "ios"]);
        // Kitsune Zero
        gametoos.insert(1291258200,vec!["win"]);
        // Kitty Tactics
        gametoos.insert(1467566601,vec!["linux", "win", "mac"]);
        // Kotel Ne Gori: A Friend of Lena Boots
        gametoos.insert(1642070196,vec!["win"]);
        // Lamplight City
        gametoos.insert(2676405412,vec!["linux", "win", "mac"]);
        // Lands of Lore - The Throne of Chaos
        gametoos.insert(71909589,vec!["dos"]);
        // LaserCat
        gametoos.insert(3647199328,vec!["win"]);
        // Learn Japanese To Survive! Hiragana Battle
        gametoos.insert(203113299,vec!["win", "mac"]);
        // Lila's Sky Ark
        gametoos.insert(875907547,vec!["linux", "win", "mac"]);
        // Lingo
        gametoos.insert(1449537939,vec!["win"]);
        // Little Racers STREET
        gametoos.insert(2011788384,vec!["win"]);
        // Lode Runner Online: The Mad Monks' Revenge
        gametoos.insert(3504603913,vec![]);
        // Long Live the Queen
        gametoos.insert(1507863460,vec!["linux", "win", "mac"]);
        // The Longest Journey
        gametoos.insert(2559734008,vec!["win", "ios"]);
        // Loom
        gametoos.insert(2592636700,vec!["linux", "win", "dos", "mac"]);
        // Lost Cities
        gametoos.insert(2843146844,vec!["linux", "win", "mac"]);
        // Luck be a Landlord
        gametoos.insert(1295220560,vec!["linux", "win", "mac"]);
        // Lycanthorn II
        gametoos.insert(567957947,vec!["win"]);
        // Malice & Greed
        gametoos.insert(258323205,vec!["win"]);
        // Master of Orion
        gametoos.insert(2365933317,vec![]);
        // Mega Man Legacy Collection
        gametoos.insert(51190182,vec!["win"]);
        // Meganoid
        gametoos.insert(1825370595,vec!["linux", "win", "mac", "ios"]);
        // Meganoid: Grandpa's Chronicles
        gametoos.insert(361491924,vec![]);
        // Mercenary Kings
        gametoos.insert(3571620251,vec!["win", "mac"]);
        // Mewnbase
        gametoos.insert(109730221,vec!["win"]);
        // Miasma: Citizens of Free Thought
        gametoos.insert(2672134974,vec!["linux", "win", "mac"]);
        // Miasma 2: Freedom Uprising
        gametoos.insert(1528724953,vec![]);
        // Micro Mages
        gametoos.insert(2796295634,vec!["win"]);
        // MidBoss
        gametoos.insert(3459136742,vec!["linux", "win", "mac"]);
        // Minecraft
        gametoos.insert(1703712106,vec!["linux", "win", "mac"]);
        // Mirrorama
        gametoos.insert(1530248213,vec!["linux", "win"]);
        // Mobius Front 83
        gametoos.insert(21084566,vec!["linux", "win", "mac"]);
        // Molek-Syntez
        gametoos.insert(1704722141,vec!["linux", "win", "mac"]);
        // Monster Outbreak
        gametoos.insert(1388189550,vec!["linux", "win", "mac"]);
        // Moonshot - The Great Espionage
        gametoos.insert(3556045612,vec!["win"]);
        // Mount Your Friends
        gametoos.insert(2365126988,vec!["win"]);
        // Mud River
        gametoos.insert(3479442855,vec!["linux", "win", "mac"]);
        // Murder Miners
        gametoos.insert(2056930988,vec!["linux", "win", "mac"]);
        // Myst
        gametoos.insert(670769280,vec!["win", "mac"]);
        // Myst III: Exile
        gametoos.insert(2246386912,vec!["win", "mac"]);
        // Neofeud
        gametoos.insert(1640898753,vec!["win"]);
        // Neoteria
        gametoos.insert(1659068938,vec![]);
        // NeuroVoider
        gametoos.insert(1353993109,vec!["linux", "win", "mac"]);
        // Nightfall Hacker
        gametoos.insert(3958280650,vec![]);
        // Northgard
        gametoos.insert(673713456,vec!["linux", "win", "mac", "ios"]);
        // Nuclear Blaze
        gametoos.insert(2481842725,vec!["win"]);
        // One Finger Death Punch
        gametoos.insert(167113526,vec!["win", "android"]);
        // Open Sorcery
        gametoos.insert(1719301452,vec!["linux", "win", "mac", "ios"]);
        // Opus Magnum
        gametoos.insert(1636371499,vec!["linux", "win", "mac"]);
        // Osmos
        gametoos.insert(1691131058,vec!["linux", "win", "mac", "android", "ios"]);
        // Overdriven Reloaded
        gametoos.insert(602065011,vec!["linux", "win", "mac"]);
        // Owlboy
        gametoos.insert(3802042723,vec!["linux", "win", "mac"]);
        // Paladin
        gametoos.insert(1798006366,vec!["linux", "win", "mac"]);
        // Panzer General
        gametoos.insert(1633304953,vec!["dos"]);
        // Paralyzed
        gametoos.insert(1797817451,vec!["linux", "win", "mac"]);
        // Penny Arcade's On the Rain-Slick Precipice of Darkness 3
        gametoos.insert(1442934177,vec!["win", "mac", "android", "ios"]);
        // Penny Arcade's On the Rain-Slick Precipice of Darkness 4
        gametoos.insert(1442683914,vec!["win", "mac"]);
        // Phoenix Force
        gametoos.insert(2948546407,vec!["win", "android", "winphone"]);
        // Planescape: Torment
        gametoos.insert(158057550,vec!["linux", "win", "mac", "ios"]);
        // PlanetFriend
        gametoos.insert(2415815545,vec!["linux", "win", "mac"]);
        // Postal
        gametoos.insert(1975193614,vec!["win", "mac"]);
        // Press X to Not Die
        gametoos.insert(2067119678,vec!["linux", "win", "mac"]);
        // Primal Light
        gametoos.insert(3770804040,vec!["linux", "win", "mac"]);
        // Primordia
        gametoos.insert(3919727532,vec!["linux", "win", "mac", "ios"]);
        // Project Crypt
        gametoos.insert(3289329542,vec!["win", "mac"]);
        // Project Heartbeat
        gametoos.insert(3936955816,vec!["linux", "win"]);
        // Quake
        gametoos.insert(2183659576,vec!["linux", "win", "dos", "mac"]);
        // Quake II
        gametoos.insert(1711967880,vec!["linux", "win", "mac"]);
        // Quake III Arena
        gametoos.insert(3398758898,vec!["linux", "win", "mac"]);
        // Quest for Infamy
        gametoos.insert(318839218,vec!["linux", "win", "mac"]);
        // Quest of Graal
        gametoos.insert(3043627757,vec!["linux", "win", "mac"]);
        // Retro City Rampage DX
        gametoos.insert(193479195,vec!["win"]);
        // Return to Castle Wolfenstein
        gametoos.insert(2159459980,vec!["linux", "win", "mac"]);
        // Rex
        gametoos.insert(2692349906,vec![]);
        // Rex Rocket
        gametoos.insert(1217074036,vec!["linux", "win", "mac"]);
        // Rise to Ruins
        gametoos.insert(3396616260,vec!["linux", "win", "mac"]);
        // Riven
        gametoos.insert(228019385,vec!["win", "mac", "ios"]);
        // Rodent and Plank: Secret Origin
        gametoos.insert(759117852,vec!["linux", "win"]);
        // Rogue Legacy
        gametoos.insert(2217939096,vec!["linux", "win", "mac"]);
        // Rogue State Revolution
        gametoos.insert(3284984257,vec!["win"]);
        // RollerCoaster Tycoon
        gametoos.insert(2239837498,vec!["win"]);
        // RollerCoaster Tycoon 2
        gametoos.insert(366494756,vec!["win"]);
        // Roma Invicta
        gametoos.insert(2601652100,vec![]);
        // Ruggnar
        gametoos.insert(738681263,vec!["win"]);
        // ROTA
        gametoos.insert(2122792745,vec![]);
        // RUN: The world in-between
        gametoos.insert(3069854191,vec!["linux", "win", "mac"]);
        // RuneScape
        gametoos.insert(779289133,vec!["linux", "win", "mac", "android", "ios"]);
        // Rushberry Mercs
        gametoos.insert(3993039567,vec![]);
        // SEGA Mega Drive and Genesis Classics
        gametoos.insert(1828657701,vec![]);
        // SJ-19 Learns To Love!
        gametoos.insert(1658453052,vec!["linux", "win"]);
        // Sokobos
        gametoos.insert(2164501758,vec!["linux", "win", "mac"]);
        // SokoChess
        gametoos.insert(2074523085,vec![]);
        // SokoChess White
        gametoos.insert(146962511,vec![]);
        // SUMICO - The Numbers Game
        gametoos.insert(2747645675,vec!["linux", "win", "mac"]);
        // Salt and Sanctuary
        gametoos.insert(2627238074,vec!["linux", "win", "mac"]);
        // Sam & Max Hit the Road
        gametoos.insert(722522936,vec!["linux", "win", "dos", "mac"]);
        // satryn deluxe
        gametoos.insert(3027209299,vec!["linux", "win"]);
        // Session Seven
        gametoos.insert(586137124,vec!["linux", "win"]);
        // The Settlers II
        gametoos.insert(2169925840,vec!["dos", "mac"]);
        // The Settlers
        gametoos.insert(3974521808,vec![]);
        // Seven Kingdoms: Ancient Adversaries
        gametoos.insert(1470641001,vec!["win"]);
        // Shadow Warrior Classic Redux
        gametoos.insert(2547716035,vec!["linux", "win", "mac", "android"]);
        // Shigatari
        gametoos.insert(4010254425,vec!["linux", "win", "mac"]);
        // Shipwreck
        gametoos.insert(2125892255,vec![]);
        // The Shivah
        gametoos.insert(3733177703,vec!["linux", "win", "mac", "android", "ios"]);
        // Shrine
        gametoos.insert(1238697931,vec!["win"]);
        // Shrine II
        gametoos.insert(449255765,vec!["linux", "win"]);
        // Signs of Life
        gametoos.insert(4017687802,vec!["linux", "win"]);
        // Simona's Requiem
        gametoos.insert(4067415938,vec!["win"]);
        // Sir Questionnaire
        gametoos.insert(865905578,vec!["win", "android", "ios"]);
        // Skulls of the Shogun
        gametoos.insert(672563997,vec!["linux", "win", "mac", "android", "ios", "winphone"]);
        // Slay the Spire
        gametoos.insert(3776946340,vec!["linux", "win", "mac", "android", "ios"]);
        // Snake Core
        gametoos.insert(1227673738,vec!["win"]);
        // Solar Rogue
        gametoos.insert(3586711541,vec!["linux", "win"]);
        // Solaroids: Prologue
        gametoos.insert(3713635070,vec!["linux", "win", "mac"]);
        // Soulcaster I & II
        gametoos.insert(4065223331,vec!["linux", "win", "mac"]);
        // SpaceChem
        gametoos.insert(940030390,vec!["linux", "win", "mac", "android", "ios"]);
        // Space Grunts
        gametoos.insert(2898300560,vec!["linux", "win", "mac", "android", "ios"]);
        // Space Grunts 2
        gametoos.insert(4206395974,vec!["linux", "win", "mac"]);
        // Space Haven
        gametoos.insert(2607736985,vec!["linux", "win", "mac"]);
        // Spaceport Hope
        gametoos.insert(2457286670,vec!["win"]);
        // SpeedRunners
        gametoos.insert(2860433649,vec!["linux", "win", "mac"]);
        // Spellbook Demonslayers
        gametoos.insert(174433159,vec!["win"]);
        // Star Wars: Jedi Knight - Jedi Academy
        gametoos.insert(1026195232,vec!["win", "mac"]);
        // Stardash
        gametoos.insert(185773087,vec![]);
        // Stardew Valley
        gametoos.insert(2245791810,vec!["linux", "win", "mac", "android", "ios"]);
        // StarPrey
        gametoos.insert(2954122711,vec!["win"]);
        // Star-Twine
        gametoos.insert(4187681019,vec!["win"]);
        // Steel Assault
        gametoos.insert(3234895239,vec!["linux", "win", "mac"]);
        // Stone Kingdoms
        gametoos.insert(268643508,vec![]);
        // Strangeland
        gametoos.insert(2843820624,vec!["linux", "win", "mac"]);
        // Strife: Veteran Edition
        gametoos.insert(1108721645,vec!["linux", "win", "mac"]);
        // Suits: A Business RPG
        gametoos.insert(1026879291,vec!["win"]);
        // Sunrider: Mask of Arcadius
        gametoos.insert(2672676346,vec!["linux", "win", "mac"]);
        // Super Amazing Wagon Adventure
        gametoos.insert(8820847,vec!["win"]);
        // Super Bernie World
        gametoos.insert(3323450609,vec!["win"]);
        // Super Blood Hockey
        gametoos.insert(1848813103,vec!["linux", "win", "mac"]);
        // Super Hexagon
        gametoos.insert(1904006542,vec!["linux", "win", "mac", "android", "ios", "blackberry"]);
        // Super Ninja Warrior Extreme
        gametoos.insert(4160219604,vec![]);
        // Super Rad Raygun
        gametoos.insert(3579077245,vec!["linux", "win", "mac"]);
        // Sword of the Stars: The Pit
        gametoos.insert(1892619583,vec!["win"]);
        // System Shock
        gametoos.insert(2137533098,vec!["dos", "mac"]);
        // Tales of Maj'Eyal
        gametoos.insert(3286093679,vec!["linux", "win", "mac"]);
        // Tanglewood
        gametoos.insert(3559422909,vec!["linux", "win", "mac"]);
        // TD Worlds
        gametoos.insert(878117955,vec!["win"]);
        // Technobabylon
        gametoos.insert(2092934091,vec!["win", "ios"]);
        // Terraria
        gametoos.insert(717044249,vec!["linux", "win", "mac", "android", "ios", "winphone"]);
        // Theia: The Crimson Eclipse
        gametoos.insert(226077421,vec!["win"]);
        // Theme Hospital
        gametoos.insert(3787425836,vec!["win", "dos"]);
        // Thukothea Defender
        gametoos.insert(1547648105,vec![]);
        // Timespinner
        gametoos.insert(1384576505,vec!["linux", "win", "mac"]);
        // Time's Up in Tiny Town
        gametoos.insert(1405076312,vec![]);
        // Titan Attacks!
        gametoos.insert(664400785,vec!["linux", "win", "mac"]);
        // Toonstruck
        gametoos.insert(1234201391,vec!["win", "dos", "mac"]);
        // TowerFall: Ascension
        gametoos.insert(3373980756,vec!["linux", "win", "mac"]);
        // Tyrian 2000
        gametoos.insert(3531862164,vec!["win", "dos"]);
        // Ultra Hat Dimension
        gametoos.insert(3756401890,vec!["linux", "win", "mac"]);
        // Ultratron
        gametoos.insert(102280636,vec!["linux", "win", "mac"]);
        // Unavowed
        gametoos.insert(2953591878,vec!["win", "mac"]);
        // Underlings
        gametoos.insert(4231456798,vec!["win"]);
        // Unexplored
        gametoos.insert(3047124949,vec!["linux", "win", "mac"]);
        // Unholy Heights
        gametoos.insert(277921868,vec!["win"]);
        // Urtuk: The Desolation
        gametoos.insert(2762679391,vec!["linux", "win", "mac"]);
        // The Useful Dead
        gametoos.insert(1004706334,vec![]);
        // Virtual Cottage
        gametoos.insert(2574776493,vec!["win"]);
        // A Virus Named Tom
        gametoos.insert(1171843366,vec!["linux", "win", "mac"]);
        // Voidrun
        gametoos.insert(3471429174,vec!["win"]);
        // Vomitoreum
        gametoos.insert(2781419886,vec!["linux", "win"]);
        // Vylan
        gametoos.insert(1361928155,vec!["win"]);
        // Weapon of Choice
        gametoos.insert(3735222255,vec!["win"]);
        // Whispers of a Machine
        gametoos.insert(3859508815,vec!["win", "mac"]);
        // Wish Project
        gametoos.insert(3764779777,vec!["win"]);
        // WizOrb
        gametoos.insert(730050274,vec!["linux", "win", "mac"]);
        // Woodland Empire
        gametoos.insert(1757161184,vec![]);
        // Word Game: Episode 0
        gametoos.insert(3423986932,vec!["win", "mac"]);
        // Wrath: Aeon of Ruin
        gametoos.insert(2399402283,vec!["linux", "win", "mac"]);
        // Wyv and Keep
        gametoos.insert(4148782727,vec!["linux", "win", "mac"]);
        // X-Com: UFO Defense (aka UFO: Enemy Unknown)
        gametoos.insert(1173623993,vec!["win", "dos"]);
        // Yarntown
        gametoos.insert(3155509741,vec!["linux", "win", "mac"]);
        // Yume Nikki
        gametoos.insert(3619239825,vec!["win"]);
        // The Zachtronics Solitaire Collection
        gametoos.insert(4128686703,vec!["linux", "win", "mac"]);
        // Zak McKracken and the Alien Mindbenders
        gametoos.insert(3237529786,vec!["dos"]);
        // Zen Bound 2
        gametoos.insert(1508890050,vec!["linux", "win", "mac"]);
        gametoos
    };
}
