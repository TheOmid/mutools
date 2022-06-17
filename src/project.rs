use std::io::Write;

// Core project type
// Contains metadata like the project data's location, and manages loading/saving of project data.
pub struct Project {
    project_data: ProjectData,
}

// Raw serializable type representing buffers, metadata, etc, contained in a project
#[derive(serde::Serialize, serde::Deserialize)]
pub struct ProjectData {}

impl ProjectData {
    pub fn load_from_file(file: std::fs::File) -> Result<ProjectData, crate::errors::DataError> {
        let bson_document_res = bson::Document::from_reader(file);
        match bson_document_res {
            Ok(bson_data) => {
                match bson::from_bson::<ProjectData>(bson::Bson::Document(bson_data)) {
                    Err(_) => Err(crate::errors::DataError::new()),
                    Ok(data) => Ok(data),
                }
            }
            _ => Err(crate::errors::DataError::new()),
        }
    }

    pub fn save_to_file(self, file: std::fs::File) -> Result<(), crate::errors::DataError> {
        let bson_res = bson::to_bson(&self);
        match bson_res {
            Ok(bson::Bson::Document(bson_data)) => match bson_data.to_writer(file) {
                Ok(_) => Ok(()),
                _ => Err(crate::errors::DataError::new()),
            },
            _ => Err(crate::errors::DataError::new()),
        }
    }
}

impl Project {
    pub fn get_project_data(&self) -> &ProjectData {
        &self.project_data
    }
}
