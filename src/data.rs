#[derive(serde::Serialize, serde::Deserialize, Clone, Debug)]
pub struct Data {
    pub personal: Option<Personal>,
    pub skills: Option<Skills>,
    pub publications: Option<Vec<Publication>>,
    pub education: Option<Vec<Education>>,
    pub jobs: Option<Vec<Job>>,
    pub projects: Option<Vec<Project>>,
}
impl Data {
    pub fn new(text: &str) -> Option<Self> {
        let data: Result<Data, serde_json::Error> = 
            serde_json::from_str(text);
        if let Err(e) = data {
            println!("Error on authorization struct parsing/building: {}", e);
            return None;
        }
        Some(data.unwrap())
    }
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug)]
pub struct Personal {
    pub name: Option<String>,
    pub email: Option<String>,
    pub website: Option<String>,
    pub phone: Option<String>,
    pub socials: Option<Vec<String>>,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug)]
pub struct Skills {
    pub languages: Option<Vec<String>>,
    pub tools: Option<Vec<String>>,
    pub topics: Option<Vec<String>>,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug)]
pub struct Publication {
    pub title: Option<String>,
    pub role: Option<String>,
    pub link: Option<String>,
    pub body: Option<Vec<String>>,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug)]
pub struct Education {
    pub title: Option<String>,
    pub location: Option<String>,
    pub duration: Option<String>,
    pub body: Option<Vec<String>>,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug)]
pub struct Job {
    pub title: Option<String>,
    pub employer: Option<String>,
    pub location: Option<String>,
    pub duration: Option<String>,
    pub body: Option<Vec<String>>,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug)]
pub struct Project {
    pub title: Option<String>,
    pub tool: Option<String>,
    pub link: Option<String>,
    pub body: Option<Vec<String>>,
}