use crate::cv::YoloDetector;
use crate::android::Android;

pub struct VillageHandler {
    android: Android,
    profiles: Vec<String>
}

impl VillageHandler {
    pub fn new(android: Android, profiles: Vec<String>) -> Self {
        let building_detector = YoloDetector::new("./assets/models/building_detector_model.onnx", 0.8);
        VillageHandler {
            android,
            profiles
        }
    }

    pub fn run(&self) {

    }

}