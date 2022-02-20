import QtQuick 2.0

Rectangle {
    width: 300
    height: 600
    Image {
        id: sightingImage
        height: 4* parent.height/5
        anchors.centerIn: parent
        anchors.bottom: menu.top
        source: client.picture
        fillMode: Image.PreserveAspectFit
    }

    Menu {
        id: menu
        anchors.bottom: parent.bottom
        menuWidth: parent.width
        menuText: "Back"
        menuHeight: (parent.height/6)
        onButtonClick: {
            pageLoader.source = "Client.qml"
        }
    }
}
