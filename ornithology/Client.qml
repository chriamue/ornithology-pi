/***************************************************************************
**
** Copyright (C) 2013 BlackBerry Limited. All rights reserved.
** Copyright (C) 2017 The Qt Company Ltd.
** Contact: https://www.qt.io/licensing/
**
** This file is part of the QtBluetooth module of the Qt Toolkit.
**
** $QT_BEGIN_LICENSE:BSD$
** Commercial License Usage
** Licensees holding valid commercial Qt licenses may use this file in
** accordance with the commercial license agreement provided with the
** Software or, alternatively, in accordance with the terms contained in
** a written agreement between you and The Qt Company. For licensing terms
** and conditions see https://www.qt.io/terms-conditions. For further
** information use the contact form at https://www.qt.io/contact-us.
**
** BSD License Usage
** Alternatively, you may use this file under the terms of the BSD license
** as follows:
**
** "Redistribution and use in source and binary forms, with or without
** modification, are permitted provided that the following conditions are
** met:
**   * Redistributions of source code must retain the above copyright
**     notice, this list of conditions and the following disclaimer.
**   * Redistributions in binary form must reproduce the above copyright
**     notice, this list of conditions and the following disclaimer in
**     the documentation and/or other materials provided with the
**     distribution.
**   * Neither the name of The Qt Company Ltd nor the names of its
**     contributors may be used to endorse or promote products derived
**     from this software without specific prior written permission.
**
**
** THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS
** "AS IS" AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT
** LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR
** A PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT
** OWNER OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL,
** SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT
** LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE,
** DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY
** THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
** (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
** OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE."
**
** $QT_END_LICENSE$
**
****************************************************************************/

import QtQuick 2.0

Rectangle {
    width: 300
    height: 600

    Component.onCompleted: {
        // Loading this page may take longer than QLEController
        // stopping with an error, go back and readjust this view
        // based on controller errors
        if (client.controllerError) {
            info.visible = false;
            menu.menuText = client.update
        }
    }

    Header {
        id: header
        anchors.top: parent.top
        headerText: "Sightings list"
    }

    Dialog {
        id: info
        anchors.centerIn: parent
        visible: true
        dialogText: "Scanning for sightings...";
    }

    Connections {
        target: client
        function onSightingsUpdated() {
            if (sightingsview.count === 0)
                info.dialogText = "No sightings found"
            else
                info.visible = false;
        }
    }

    Connections {
        target: device

        function onDisconnected() {
            pageLoader.source = "main.qml"
        }
    }

    ListView {
        id: sightingsview
        width: parent.width
        anchors.top: header.bottom
        anchors.bottom: sightingImage.top
        model: client.sightingsList
        clip: true

        delegate: Rectangle {
            id: servicebox
            height:100
            color: "lightsteelblue"
            border.width: 2
            border.color: "black"
            radius: 5
            width: sightingsview.width
            Component.onCompleted: {
                info.visible = false
            }

            MouseArea {
                anchors.fill: parent
                onClicked: {
                    client.loadImage(modelData.sightingUuid);
                }
            }

            Label {
                id: sightingsSpecies
                textContent: modelData.sightingSpecies
                anchors.top: parent.top
                anchors.topMargin: 5
            }

            Label {
                id: sightingsDatetime
                font.pointSize: sightingsSpecies.font.pointSize * 0.5
                textContent: modelData.sightingDatetime
                anchors.top: sightingsSpecies.bottom
                anchors.topMargin: 5
            }

            Label {
                id: sightingsUuid
                font.pointSize: sightingsSpecies.font.pointSize * 0.5
                textContent: modelData.sightingUuid
                anchors.bottom: servicebox.bottom
                anchors.bottomMargin: 5
            }


        }
    }

    Image {
        id: sightingImage
        height: parent.height/5
        x: parent.width / 2 - width / 2
        anchors.bottom: menu.top
        source: client.picture
        fillMode: Image.PreserveAspectFit

        MouseArea {
            anchors.fill: parent
            onClicked: {
                pageLoader.source = "ZoomImage.qml";
            }
        }
    }

    Menu {
        id: menu
        anchors.bottom: parent.bottom
        menuWidth: parent.width
        menuText: device.update
        menuHeight: (parent.height/6)
        onButtonClick: {
            client.disconnect()
            pageLoader.source = "main.qml"
            device.update = "Search"
        }
    }
}
