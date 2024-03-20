from opencv_camera_detector import camera_list


def test_camera_list():
    for info in camera_list():
        assert isinstance(info, dict)
        assert int(info["index"]) >= 0
