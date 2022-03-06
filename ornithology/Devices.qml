import QtQuick
import QtQuick.Controls 2

Rectangle {
    width: 300
    height: 600

    SearchMenu {
        id: menu
        anchors.top: parent.top
        menuWidth: parent.width
        menuHeight: (parent.height/6)
        menuText: device.update

        onButtonClick: {
            if (device.state) {
                device.deviceScanFinished();
            } else {
                device.startDeviceDiscovery();
                if (device.state) {
                    info.dialogText = "Searching...";
                    info.visible = true;
                }
            }
        }

        Image {
            id: bluetoothImage
            x: 20
            y: 10
            width: 50
            height: 50
            source: "resources/bluetooth.png"
            fillMode: Image.PreserveAspectFit
        }
    }

    InfoDialog {
        id: info
        anchors.centerIn: parent
        visible: false
    }

    ListView {
        id: theListView
        width: parent.width
        clip: true

        anchors.top: menu.bottom
        anchors.bottom: parent.bottom
        model: device.devicesList

        ScrollBar.vertical: ScrollBar { id: scrollBar }

        delegate: Rectangle {
            id: box
            height:100
            width: theListView.width
            color: "lightsteelblue"
            border.width: 2
            border.color: "black"
            radius: 5

            Component.onCompleted: {
                info.visible = false;
            }

            MouseArea {
                anchors.fill: parent
                onClicked: {
                    client.connect(modelData.deviceAddress);
                    pageLoader.source = "Client.qml"
                }
            }

            CustomLabel {
                id: deviceName
                textContent: modelData.deviceName
                anchors.top: parent.top
                anchors.topMargin: 5
            }

            CustomLabel {
                id: deviceAddress
                textContent: modelData.deviceAddress
                font.pointSize: deviceName.font.pointSize*0.7
                anchors.bottom: box.bottom
                anchors.bottomMargin: 5
            }
        }
    }
}
