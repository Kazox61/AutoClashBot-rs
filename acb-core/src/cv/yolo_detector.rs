use image::DynamicImage;
use yolov8_rs::{Args, YOLOv8, YOLOTask};
use crate::cv::rect::Rectangle;


pub struct DetectorResult {
    pub cls: usize,
    pub conf: f32,
    pub name: String,
    pub rect: Rectangle,
    pub rectn: Rectangle
}

pub struct YoloDetector {
    yolov8: YOLOv8
}

impl YoloDetector {
    pub fn new(model_path: &str, conf: f32) -> YoloDetector {
        let args = Args {
            model: model_path.to_string(),
            device_id: 0,
            trt: false,
            cuda: false,
            batch: 1,
            batch_min: 1,
            batch_max: 32,
            fp16: false,
            task: Some(YOLOTask::Detect),
            nc: None,
            nk: None,
            nm: None,
            width: None,
            height: None,
            conf,
            iou: 0.45,
            kconf: 0.55,
            plot: true,
            profile: false,
        };

        YoloDetector {
            yolov8: YOLOv8::new(args).unwrap()
        }
    }

    pub fn predict(&mut self, image: DynamicImage, conf: f32) -> Vec<DetectorResult> {
        let img_width = image.width() as f32;
        let img_height = image.height() as f32;
        self.yolov8.set_conf(conf);
        let result = self.yolov8.run(&vec![image]).unwrap();

        let mut predict_results = vec![];
        let names = self.yolov8.names();

        for yolo_result in result {
            let boxes = match yolo_result.bboxes() {
                None => return vec![],
                Some(boxes) => boxes
            };
            for bbox in boxes {
                let dr = DetectorResult {
                    cls: bbox.id(),
                    conf: bbox.confidence(),
                    name: names[bbox.id()].clone(),
                    rect: Rectangle::new(bbox.xmin(), bbox.ymin(), bbox.width(), bbox.height()),
                    rectn: Rectangle::new(bbox.xmin() / img_width, bbox.ymin() / img_height, bbox.width() / img_width, bbox.height() / img_height)
                };

                predict_results.push(dr);
            }
        }
        predict_results
    }
}