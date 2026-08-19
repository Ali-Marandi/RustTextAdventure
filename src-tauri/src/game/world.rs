use super::types::*;

// ============================================================
// World Builder - Creates the entire game world
// ============================================================

pub fn create_world() -> Vec<Room> {
    let mut rooms = Vec::new();

    // === ENTRANCE AREA ===
    rooms.push(Room {
        id: "entrance".into(),
        name: "Laboratory Entrance".into(),
        description: "You stand before the rusted doors of the Prometheus Research Facility. "
            .to_string()
            + "Flickering neon signs spell out warnings in multiple languages. "
            + "The air hums with electromagnetic energy.",
        detailed_description: "The entrance hall of the Prometheus Research Facility is a cavernous space "
            .to_string()
            + "dominated by a massive blast door that has been wedged partially open. "
            + "The walls are lined with faded safety posters and warning signs, some peeling "
            + "away to reveal older markings beneath. A reception desk sits abandoned, "
            + "its surface covered in dust and scattered papers. Emergency lighting casts "
            + "an eerie red glow across the space, making shadows dance along the walls. "
            + "The floor is cracked concrete with strange circuit-like patterns etched into it, "
            + "glowing faintly with a pulsing blue light.",
        exits: vec![
            Exit { direction: Direction::North, target_room: "main_hall".into(), description: Some("Heavy steel doors lead into the main hall".into()), locked: false, required_item: None, hidden: false },
        ],
        items: vec!["facility_keycard".into()],
        flags: vec![],
        visited: false, visit_count: 0,
        ambient_sound: Some("wind_howl".into()),
        lighting: LightingLevel::Dim,
    });

    // === MAIN HALL ===
    rooms.push(Room {
        id: "main_hall".into(),
        name: "Main Hall".into(),
        description: "A vast hexagonal hall with a towering ceiling. Six corridors radiate outward "
            .to_string()
            + "like spokes of a wheel. A massive holographic display flickers in the center, "
            + "showing fragmented facility maps and warning messages.",
        detailed_description:
            "The Main Hall is the architectural heart of the Prometheus Facility. ".to_string()
                + "Six massive corridors radiate from this central hub, each marked with "
                + "color-coded signage. The floor is polished black marble with embedded "
                + "LED strips forming intricate circuit patterns that pulse with soft light. "
                + "In the exact center stands a three-meter holographic projector, currently "
                + "displaying a corrupted map of the facility - sections blink in and out of "
                + "existence. The walls are lined with shattered display cases that once held "
                + "scientific awards and prototypes. Ceiling panels hang at dangerous angles, "
                + "revealing a lattice of cables and conduits above.",
        exits: vec![
            Exit {
                direction: Direction::South,
                target_room: "entrance".into(),
                description: Some("Back to the entrance".into()),
                locked: false,
                required_item: None,
                hidden: false,
            },
            Exit {
                direction: Direction::East,
                target_room: "library".into(),
                description: Some("A corridor lined with bookshelves".into()),
                locked: false,
                required_item: None,
                hidden: false,
            },
            Exit {
                direction: Direction::West,
                target_room: "server_room".into(),
                description: Some("A heavy security door with a card reader".into()),
                locked: false,
                required_item: None,
                hidden: false,
            },
            Exit {
                direction: Direction::North,
                target_room: "research_lab".into(),
                description: Some("Double doors marked 'AUTHORIZED PERSONNEL ONLY'".into()),
                locked: true,
                required_item: Some("facility_keycard".into()),
                hidden: false,
            },
            Exit {
                direction: Direction::Up,
                target_room: "observation_deck".into(),
                description: Some("A spiral staircase ascending into darkness".into()),
                locked: false,
                required_item: None,
                hidden: false,
            },
            Exit {
                direction: Direction::Down,
                target_room: "basement_storage".into(),
                description: Some("A service ladder descending into the depths".into()),
                locked: false,
                required_item: None,
                hidden: false,
            },
        ],
        items: vec![],
        flags: vec![],
        visited: false,
        visit_count: 0,
        ambient_sound: Some("ambient_hum".into()),
        lighting: LightingLevel::Bright,
    });

    // === LIBRARY ===
    rooms.push(Room {
        id: "library".into(),
        name: "Digital Archive Library".into(),
        description: "Thousands of holographic books float in organized rows. A terminal hums "
            .to_string()
            + "in the corner, its screen displaying a search interface. The air smells of "
            + "ozone and old paper - an odd combination.",
        detailed_description: "The Digital Archive Library is a breathtaking space where technology "
            .to_string()
            + "meets knowledge. Holographic book displays float in perfect rows, each one "
            + "containing thousands of digitized volumes. Soft blue light emanates from each "
            + "floating shelf, creating a serene, otherworldly atmosphere. A curved desk in the "
            + "corner holds an active terminal with a search interface, its screen casting "
            + "a gentle glow. Comfortable reading nooks are carved into the walls, each with "
            + "adjustable lighting. The floor is made of a translucent material that shows "
            + "moving data streams beneath your feet. A large portrait of the facility's "
            + "founder, Dr. Elena Prometheus, hangs above the main desk.",
        exits: vec![
            Exit { direction: Direction::West, target_room: "main_hall".into(), description: Some("Back to the main hall".into()), locked: false, required_item: None, hidden: false },
            Exit { direction: Direction::North, target_room: "directors_office".into(), description: Some("A locked door with a keypad".into()), locked: true, required_item: Some("office_code".into()), hidden: false },
        ],
        items: vec!["research_journal".into(), "usb_drive".into()],
        flags: vec![],
        visited: false, visit_count: 0,
        ambient_sound: Some("library_quiet".into()),
        lighting: LightingLevel::Bright,
    });

    // === SERVER ROOM ===
    rooms.push(Room {
        id: "server_room".into(),
        name: "Server Room".into(),
        description: "Racks of humming servers stretch from floor to ceiling. Cooling fans "
            .to_string()
            + "create a constant breeze. A central console displays system diagnostics. "
            + "Blue and green LEDs create a mesmerizing light show.",
        detailed_description: "The Server Room is a cathedral of computing power. Towering racks "
            .to_string()
            + "of servers stretch from floor to ceiling in neat rows, each one humming "
            + "with processing power. Massive cooling units built into the walls create "
            + "a constant cold breeze that carries the scent of ozone. The central "
            + "operations console displays real-time system diagnostics on multiple "
            + "monitors - most show warning indicators. Thousands of LEDs blink in "
            + "synchronized patterns, creating a mesmerizing light show that shifts "
            + "between blue and green. Cable trays run along the ceiling like mechanical "
            + "vines, thick bundles of fiber optic cables connecting the servers in a "
            + "complex web of data pathways.",
        exits: vec![
            Exit {
                direction: Direction::East,
                target_room: "main_hall".into(),
                description: Some("Back to the main hall".into()),
                locked: false,
                required_item: None,
                hidden: false,
            },
            Exit {
                direction: Direction::North,
                target_room: "network_hub".into(),
                description: Some("A reinforced door with a biometric lock".into()),
                locked: true,
                required_item: Some("biometric_key".into()),
                hidden: false,
            },
        ],
        items: vec!["network_cable".into()],
        flags: vec!["servers_overloaded".into()],
        visited: false,
        visit_count: 0,
        ambient_sound: Some("server_hum".into()),
        lighting: LightingLevel::Bright,
    });

    // === RESEARCH LAB ===
    rooms.push(Room {
        id: "research_lab".into(),
        name: "Advanced Research Laboratory".into(),
        description: "A state-of-the-art laboratory filled with exotic equipment. Broken "
            .to_string()
            + "glassware and spilled chemicals suggest a hasty evacuation. A large "
            + "experimentation chamber dominates the far wall.",
        detailed_description: "The Advanced Research Laboratory is a scene of organized chaos. "
            .to_string()
            + "State-of-the-art equipment lines the walls: electron microscopes, spectral "
            + "analyzers, and devices of such exotic design that their purpose is unclear. "
            + "Broken glassware litters the workbenches, and pools of colorful chemicals "
            + "have dried into crystalline patterns on the surfaces. A large experimentation "
            + "chamber with reinforced glass walls dominates the far end of the room, "
            + "its interior scorched and blackened. Whiteboards covered in complex "
            + "equations and diagrams cover every available wall space. A personal "
            + "workstation in the corner still has an active session, its screen showing "
            + "the last experiment's data.",
        exits: vec![
            Exit {
                direction: Direction::South,
                target_room: "main_hall".into(),
                description: Some("Back to the main hall".into()),
                locked: false,
                required_item: None,
                hidden: false,
            },
            Exit {
                direction: Direction::East,
                target_room: "chemical_storage".into(),
                description: Some("A reinforced door with hazard warnings".into()),
                locked: false,
                required_item: None,
                hidden: false,
            },
            Exit {
                direction: Direction::North,
                target_room: "experiment_chamber".into(),
                description: Some("An airlock door with a warning sign".into()),
                locked: true,
                required_item: Some("hazard_suit".into()),
                hidden: false,
            },
        ],
        items: vec![
            "hazard_suit".into(),
            "lab_notes".into(),
            "chemical_formula".into(),
        ],
        flags: vec![],
        visited: false,
        visit_count: 0,
        ambient_sound: Some("lab_buzz".into()),
        lighting: LightingLevel::Bright,
    });

    // === OBSERVATION DECK ===
    rooms.push(Room {
        id: "observation_deck".into(),
        name: "Observation Deck".into(),
        description: "A panoramic viewing platform overlooking the entire facility through "
            .to_string()
            + "reinforced glass windows. Control panels line the walls, most dark. "
            + "A telescope points toward a distant signal tower.",
        detailed_description: "The Observation Deck offers a breathtaking view of the facility "
            .to_string()
            + "and the surrounding landscape. Massive reinforced glass windows wrap "
            + "around the curved walls, providing an unobstructed panoramic view. Below, "
            + "you can see the facility's various wings spreading out like a geometric "
            + "flower. In the distance, a communications tower rises against the horizon, "
            + "its warning light blinking rhythmically. Control panels line the interior "
            + "walls, most dark and unresponsive, but a few still show flickering data. "
            + "A high-powered telescope is mounted on a rotating platform, currently "
            + "pointed at the communications tower. Comfortable chairs suggest this was "
            + "once a place for scientists to take breaks and think.",
        exits: vec![Exit {
            direction: Direction::Down,
            target_room: "main_hall".into(),
            description: Some("Spiral staircase back down".into()),
            locked: false,
            required_item: None,
            hidden: false,
        }],
        items: vec!["signal_flare".into(), "binoculars".into()],
        flags: vec![],
        visited: false,
        visit_count: 0,
        ambient_sound: Some("wind_quiet".into()),
        lighting: LightingLevel::Bright,
    });

    // === BASEMENT STORAGE ===
    rooms.push(Room {
        id: "basement_storage".into(),
        name: "Basement Storage".into(),
        description: "Dusty shelves piled high with crates and equipment. The air is thick "
            .to_string()
            + "and stale. Cobwebs drape from the low ceiling. Something glints in the "
            + "corner.",
        detailed_description: "The Basement Storage is a labyrinth of towering metal shelves "
            .to_string()
            + "stacked with dusty crates, old equipment, and forgotten supplies. The "
            + "ceiling is low and crisscrossed with pipes and ductwork that occasionally "
            + "drip condensation. Cobwebs drape like curtains between the shelves, and "
            + "the air is thick with the smell of dust and machine oil. Flickering fluorescent "
            + "lights cast harsh shadows that seem to move on their own. In one corner, "
            + "a workbench holds partially disassembled equipment and tools. A large "
            + "shipping crate marked 'FRAGILE - PROMETHEUS PROJECT' sits against the "
            + "far wall, its lid slightly ajar.",
        exits: vec![
            Exit {
                direction: Direction::Up,
                target_room: "main_hall".into(),
                description: Some("Service ladder back up".into()),
                locked: false,
                required_item: None,
                hidden: false,
            },
            Exit {
                direction: Direction::East,
                target_room: "generator_room".into(),
                description: Some("A heavy door with electrical warning signs".into()),
                locked: true,
                required_item: Some("circuit_breaker".into()),
                hidden: false,
            },
        ],
        items: vec![
            "circuit_breaker".into(),
            "old_radio".into(),
            "toolbox".into(),
        ],
        flags: vec![],
        visited: false,
        visit_count: 0,
        ambient_sound: Some("dripping".into()),
        lighting: LightingLevel::Dim,
    });

    // === DIRECTOR'S OFFICE ===
    rooms.push(Room {
        id: "directors_office".into(),
        name: "Director's Office".into(),
        description: "An opulent office contrasting sharply with the facility's industrial "
            .to_string()
            + "aesthetic. Mahogany desk, leather chairs, and original artwork suggest "
            + "considerable power and wealth. A hidden safe is set into the wall.",
        detailed_description: "The Director's Office is an island of luxury within the stark "
            .to_string()
            + "industrial complex. A massive mahogany desk dominates the center of the "
            + "room, its surface immaculately clean except for a nameplate reading "
            + "'Dr. Helena Prometheus - Director'. Behind the desk, a floor-to-ceiling "
            + "window (now shuttered) once offered a view of the facility. Original artwork "
            + "hangs on the walls - paintings that seem oddly out of place in a research "
            + "facility. A leather armchair sits behind the desk, and two visitor chairs "
            + "face it. A hidden safe is set into the wall behind a painting, and a "
            + "personal computer sits on a side table, its screen locked.",
        exits: vec![Exit {
            direction: Direction::South,
            target_room: "library".into(),
            description: Some("Back to the library".into()),
            locked: false,
            required_item: None,
            hidden: false,
        }],
        items: vec!["director_key".into(), "photograph".into()],
        flags: vec![],
        visited: false,
        visit_count: 0,
        ambient_sound: Some("clock_tick".into()),
        lighting: LightingLevel::Bright,
    });

    // === NETWORK HUB ===
    rooms.push(Room {
        id: "network_hub".into(),
        name: "Network Hub".into(),
        description: "The nerve center of the facility's communications. Banks of network "
            .to_string()
            + "switches and routers fill the room. A central terminal displays "
            + "intercepted messages and encrypted communications.",
        detailed_description: "The Network Hub is the communications nerve center of the "
            .to_string()
            + "Prometheus Facility. Banks of network switches and routers fill every "
            + "available surface, their status lights creating a mesmerizing cascade of "
            + "green, amber, and red. A central terminal displays a constant stream of "
            + "data - intercepted messages, encrypted communications, and system logs. "
            + "The room is kept at a precisely controlled temperature, and the air "
            + "conditioning hums steadily. A large world map on one wall shows the "
            + "facility's connections to other research stations around the globe, most "
            + "marked with red 'OFFLINE' indicators. A powerful decryption workstation "
            + "sits in the corner, its multiple monitors displaying complex algorithms.",
        exits: vec![Exit {
            direction: Direction::South,
            target_room: "server_room".into(),
            description: Some("Back to the server room".into()),
            locked: false,
            required_item: None,
            hidden: false,
        }],
        items: vec!["decrypted_files".into(), "biometric_key".into()],
        flags: vec![],
        visited: false,
        visit_count: 0,
        ambient_sound: Some("data_stream".into()),
        lighting: LightingLevel::Bright,
    });

    // === CHEMICAL STORAGE ===
    rooms.push(Room {
        id: "chemical_storage".into(),
        name: "Chemical Storage".into(),
        description: "Rows of sealed containers and safety cabinets. Warning labels cover "
            .to_string()
            + "every surface. A fume hood in the corner contains an active experiment. "
            + "The smell of chemicals is overpowering.",
        detailed_description: "The Chemical Storage room is a carefully organized space filled "
            .to_string()
            + "with potential danger. Rows of sealed containers line the walls, each "
            + "labeled with detailed chemical formulas and hazard warnings in multiple "
            + "languages. Safety cabinets with reinforced glass doors house the most "
            + "dangerous substances. A fume hood in the corner contains what appears to "
            + "be an active experiment - a beaker of luminous green liquid connected to "
            + "a complex apparatus. The smell of chemicals is strong but not unpleasant - "
            + "like a hospital mixed with a swimming pool. Emergency showers and eye "
            + "wash stations are positioned at regular intervals. A spill containment "
            + "kit sits near the door, recently used.",
        exits: vec![Exit {
            direction: Direction::West,
            target_room: "research_lab".into(),
            description: Some("Back to the research lab".into()),
            locked: false,
            required_item: None,
            hidden: false,
        }],
        items: vec!["antidote".into(), "strange_compound".into()],
        flags: vec![],
        visited: false,
        visit_count: 0,
        ambient_sound: Some("bubbling".into()),
        lighting: LightingLevel::Bright,
    });

    // === EXPERIMENT CHAMBER ===
    rooms.push(Room {
        id: "experiment_chamber".into(),
        name: "Experiment Chamber".into(),
        description: "A massive sealed chamber with reinforced walls. Scorch marks and "
            .to_string()
            + "cracked observation windows tell of powerful experiments. In the center, "
            + "a strange device pulses with an otherworldly energy.",
        detailed_description: "The Experiment Chamber is a massive spherical room with walls "
            .to_string()
            + "of reinforced titanium alloy. Scorch marks cover every surface, and the "
            + "observation windows are cracked but holding. In the exact center of the "
            + "chamber, a strange device - part machine, part crystal formation - pulses "
            + "with an otherworldly blue-white energy. The air around it shimmers and "
            + "distorts, as if reality itself is being bent. Equipment racks around the "
            + "perimeter hold monitoring devices, most damaged or destroyed. A control "
            + "panel on one wall still has power, its screen showing energy readings "
            + "that spike dangerously high. The floor has a complex geometric pattern "
            + "etched into it, matching the energy patterns of the central device.",
        exits: vec![
            Exit {
                direction: Direction::South,
                target_room: "research_lab".into(),
                description: Some("Back through the airlock".into()),
                locked: false,
                required_item: None,
                hidden: false,
            },
            Exit {
                direction: Direction::North,
                target_room: "escape_tunnel".into(),
                description: Some("A concealed door revealed by the device's energy".into()),
                locked: true,
                required_item: Some("prometheus_core".into()),
                hidden: true,
            },
        ],
        items: vec!["energy_cell".into()],
        flags: vec!["device_active".into()],
        visited: false,
        visit_count: 0,
        ambient_sound: Some("energy_pulse".into()),
        lighting: LightingLevel::Bright,
    });

    // === GENERATOR ROOM ===
    rooms.push(Room {
        id: "generator_room".into(),
        name: "Generator Room".into(),
        description: "A massive electrical generator dominates this room. It's currently "
            .to_string()
            + "running at partial capacity. Control panels show power distribution "
            + "across the facility. Sparks occasionally fly from damaged conduits.",
        detailed_description: "The Generator Room houses the facility's primary power source - "
            .to_string()
            + "a massive fusion generator that takes up most of the space. Currently, "
            + "it's running at only 40% capacity, its characteristic blue-white glow "
            + "dimmed and flickering. Control panels on a raised platform show the power "
            + "distribution across the facility - several sections are marked as offline. "
            + "Sparks occasionally fly from damaged conduits that run along the walls "
            + "and ceiling. The room is hot and humid, with the constant hum of the "
            + "generator providing a deep bass undertone to all other sounds. A "
            + "maintenance log lies open on one of the control consoles.",
        exits: vec![Exit {
            direction: Direction::West,
            target_room: "basement_storage".into(),
            description: Some("Back to the basement".into()),
            locked: false,
            required_item: None,
            hidden: false,
        }],
        items: vec!["power_cell".into(), "maintenance_manual".into()],
        flags: vec!["generator_partial".into()],
        visited: false,
        visit_count: 0,
        ambient_sound: Some("generator_hum".into()),
        lighting: LightingLevel::Dim,
    });

    // === ESCAPE TUNNEL ===
    rooms.push(Room {
        id: "escape_tunnel".into(),
        name: "Escape Tunnel".into(),
        description: "A long, narrow tunnel stretching into darkness. Emergency lights "
            .to_string()
            + "provide sparse illumination. The walls are rough-hewn rock, suggesting "
            + "this was carved in great haste. Fresh air drifts from ahead.",
        detailed_description: "The Escape Tunnel is a desperate passage carved through solid "
            .to_string()
            + "rock. It stretches for hundreds of meters into the earth, its rough-hewn "
            + "walls telling of hasty construction. Emergency lights embedded in the ceiling "
            + "provide sparse, reddish illumination at regular intervals. The air is cool "
            + "and damp, with a noticeable draft coming from the direction of the exit. "
            + "Footprints in the dust on the floor suggest others have used this route "
            + "before. Pipes and cables run along the ceiling, some broken and dangling. "
            + "Graffiti on the walls - messages of hope and fear from those who passed "
            + "through before you.",
        exits: vec![
            Exit {
                direction: Direction::South,
                target_room: "experiment_chamber".into(),
                description: Some("Back to the experiment chamber".into()),
                locked: false,
                required_item: None,
                hidden: false,
            },
            Exit {
                direction: Direction::North,
                target_room: "exit_chamber".into(),
                description: Some("Light visible at the end of the tunnel".into()),
                locked: false,
                required_item: None,
                hidden: false,
            },
        ],
        items: vec!["final_note".into()],
        flags: vec![],
        visited: false,
        visit_count: 0,
        ambient_sound: Some("echo".into()),
        lighting: LightingLevel::Dim,
    });

    // === EXIT CHAMBER (Final Room) ===
    rooms.push(Room {
        id: "exit_chamber".into(),
        name: "Surface Exit".into(),
        description: "You emerge into a small underground chamber open to the night sky. "
            .to_string()
            + "Stars glitter overhead. The facility's communication tower stands nearby, "
            + "its signal beacon flashing. Freedom is within reach.",
        detailed_description: "The Surface Exit is a small natural chamber that opens to the "
            .to_string()
            + "night sky through a fissure in the rock ceiling. Stars glitter overhead "
            + "with brilliant clarity, undimmed by any light pollution. The facility's "
            + "communication tower stands on a nearby ridge, its signal beacon flashing "
            + "rhythmically against the darkness. Fresh, clean air fills your lungs for "
            + "the first time in what feels like forever. Wildflowers have managed to "
            + "grow around the edges of the chamber, a stark contrast to the artificial "
            + "environment you've left behind. A path leads away from the facility, and "
            + "in the distance, you can see the lights of a town. You are free.",
        exits: vec![Exit {
            direction: Direction::South,
            target_room: "escape_tunnel".into(),
            description: Some("Back into the tunnel".into()),
            locked: false,
            required_item: None,
            hidden: false,
        }],
        items: vec![],
        flags: vec!["game_complete".into()],
        visited: false,
        visit_count: 0,
        ambient_sound: Some("nature_night".into()),
        lighting: LightingLevel::Bright,
    });

    rooms
}

// Map positions for each room (for the minimap)
pub fn get_map_positions() -> std::collections::HashMap<String, (f32, f32)> {
    let mut positions = std::collections::HashMap::new();
    positions.insert("entrance".into(), (5.0, 8.0));
    positions.insert("main_hall".into(), (5.0, 6.0));
    positions.insert("library".into(), (7.0, 6.0));
    positions.insert("server_room".into(), (3.0, 6.0));
    positions.insert("research_lab".into(), (5.0, 4.0));
    positions.insert("observation_deck".into(), (5.0, 4.0));
    positions.insert("basement_storage".into(), (5.0, 8.0));
    positions.insert("directors_office".into(), (9.0, 4.0));
    positions.insert("network_hub".into(), (1.0, 4.0));
    positions.insert("chemical_storage".into(), (7.0, 4.0));
    positions.insert("experiment_chamber".into(), (5.0, 2.0));
    positions.insert("generator_room".into(), (7.0, 8.0));
    positions.insert("escape_tunnel".into(), (5.0, 0.5));
    positions.insert("exit_chamber".into(), (5.0, -1.0));
    positions
}
