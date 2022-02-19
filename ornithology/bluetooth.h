#ifndef BLUETOOTH_H
#define BLUETOOTH_H

#include <QObject>
#include <QtBluetooth/QBluetoothAddress>
#include <QtBluetooth/QBluetoothDeviceInfo>
#include <QtBluetooth/QBluetoothDeviceDiscoveryAgent>

class Bluetooth : public QObject
{
    Q_OBJECT
public:
    explicit Bluetooth(QObject *parent = nullptr);
    QStringList m_deviceNames;
    Q_INVOKABLE QStringList getDeviceNames();

public slots:
    void startDeviceDiscovery();

private slots:
    void deviceDiscovered(const QBluetoothDeviceInfo &device);

signals:
    void newDevice();

private:
    QBluetoothDeviceDiscoveryAgent *deviceDiscoveryAgent = nullptr;
    QMap<QString, QBluetoothDeviceInfo> deviceMap;

};

#endif // BLUETOOTH_H
