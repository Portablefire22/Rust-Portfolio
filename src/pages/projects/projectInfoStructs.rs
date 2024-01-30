/*
 *
 *  Holds the basic information about a project
 *  More details are located on their .rs file
 *
 */

pub struct Project {
    pub id: usize,
    pub title: String,
    pub year: u16,
    pub isComplete: bool,
    pub desc: String,
    pub url: String,
}

pub fn getProjects() -> Vec<Project> {
    vec![
        Project {
            id: 0,
            title: "KittyGoesMeow".to_string(),
            year: 2024,
            isComplete: false,
            desc: "Portfolio website so I can show off and increase my ego >:3.".to_string(),
            url: "PLACEHOLDER".to_string(),
        },
        Project {
            id: 1,
            title: "Vulkan Voxel Engine".to_string(),
            year: 2023,
            isComplete: false,
            desc: "Vulkan Voxel Engine, VVE, is an attempt to create a voxel engine using
                Vulkan and C++."
                .to_string(),
            url: "PLACEHOLDER".to_string(),
        },
        Project {
            id: 3,
            title: "League of Legends Stat Website".to_string(),
            year: 2023,
            isComplete: false,
            desc: "Website created using Rust & Rocket.rs that tracks stats for matches of
                 the game 'League of Legends'."
                .to_string(),
            url: "PLACEHOLDER".to_string(),
        },
        Project {
            id: 4,
            title: "Computer Craft Web Server Project".to_string(),
            year: 2023,
            isComplete: false,
            desc: "An unholy combination of the Minecraft mod 'ComputerCraft' and a Node.JS 
                web server to provide 3D mapping of tunnels that were mined by automated 
                turtles, with further expansion to provide a mass storage system using only 
                the small chests provided by the vanilla game."
                .to_string(),
            url: "PLACEHOLDER".to_string(),
        },
    ]
}
