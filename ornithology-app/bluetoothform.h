#ifndef BLUETOOTHFORM_H
#define BLUETOOTHFORM_H

#include <QWidget>
#include <QListWidgetItem>
#include <QtBluetooth/QBluetoothSocket>
#include <QtBluetooth/QBluetoothDeviceDiscoveryAgent>
#include <QtBluetooth/QBluetoothServiceDiscoveryAgent>
#include <QtBluetooth/QBluetoothDeviceInfo>
#include <QtBluetooth/QBluetoothServiceInfo>
#include "sightingform.h"

namespace Ui {
class BluetoothForm;
}

class BluetoothForm : public QWidget
{
    Q_OBJECT

public:
    explicit BluetoothForm(QWidget *parent = nullptr);
    ~BluetoothForm();

private slots:
    void on_pushButton_clicked();
    void startDeviceDiscovery();
    void deviceDiscovered(const QBluetoothDeviceInfo &device);
    void connectDevice(const QBluetoothDeviceInfo &device);
    void on_clientReady();
    void on_socketError(QBluetoothSocket::SocketError error);

    void on_deviceList_itemClicked(QListWidgetItem *item);

    void on_commandLinkButton_clicked();

    void on_viewClicked(QString uuid);

private:
    Ui::BluetoothForm *ui;
    QBluetoothSocket *socket = nullptr;
    QBluetoothDeviceDiscoveryAgent *deviceDiscoveryAgent = nullptr;
    QBluetoothServiceDiscoveryAgent *serviceDiscoveryAgent = nullptr;
    QMap<QString, QBluetoothDeviceInfo> deviceMap;
    QMap<QString, SightingForm*> sightings;
    QByteArray currentLine;
};

#endif // BLUETOOTHFORM_H
