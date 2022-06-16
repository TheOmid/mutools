// Core project type
// Contains metadata like the project data's location, and manages loading/saving of project data.
pub struct Project {
    data: ProjectData,
}

// Raw serializable type representing buffers, metadata, etc, contained in a project
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ProjectData {}

impl ProjectData {
    pub fn load_from_file(file: std::fs::File) -> Result<(), crate::errors::DataError> {
        Ok(())
    }

    pub fn save_to_file(file: std::fs::File) -> Result<(), crate::errors::DataError> {
        Ok(())
    }
}
