# opencvutil

OpenCV で利用できるカメラの情報を返すライブラリです。

## 使い方

```python
import cv2
import opencvutil

# カメラの情報を取得
infos = opencvutil.camera_list()

# カメラの情報を表示し、ユーザーに選択させる
for i, info in enumerate(infos):
    print(f"Index {info['index']}: {info['name']}")

# カメラを選択
camera_index = int(input("使用するカメラの Index を指定してください: "))

# OpenCV でカメラを開く
cap = cv2.VideoCapture(camera_index)
while cap.isOpened():
    ret, frame = cap.read()
    if not ret:
        break

    # 画像を表示
    cv2.imshow("frame", frame)

    if cv2.waitKey(1) & 0xFF == ord("q"):
        break

cap.release()
cv2.destroyAllWindows()
```
