# opencv-camera-detector

OpenCV で利用できるカメラの情報を返すライブラリです。

## 使い方

```python
import cv2
import opencv_camera_detector

# カメラの情報を取得
infos = opencv_camera_detector.camera_list()

# カメラの情報を表示し、ユーザーに選択させる
for i, info in enumerate(infos):
    print(f"Index {info["index"]}: {info["name"]}")

# カメラを選択
camera_index = int(input("使用するカメラの Index を指定してください: "))

# OpenCV でカメラを開く
cap = cv2.VideoCapture(camera_index)
```
