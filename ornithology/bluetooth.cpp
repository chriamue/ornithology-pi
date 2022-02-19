#include "bluetooth.h"
#include <QDebug>

Bluetooth::Bluetooth(QObject *parent)
    : QObject{parent}
{

}

QStringList Bluetooth::getDeviceNames()
{
    return this->m_deviceNames;
}

void Bluetooth::startDeviceDiscovery()
{
    deviceDiscoveryAgent = new QBluetoothDeviceDiscoveryAgent(this);
    connect(deviceDiscoveryAgent, SIGNAL(deviceDiscovered(QBluetoothDeviceInfo)),
            this, SLOT(deviceDiscovered(QBluetoothDeviceInfo)));

    deviceDiscoveryAgent->start();
}

void Bluetooth::deviceDiscovered(const QBluetoothDeviceInfo &device)
{
    qDebug() << "Found new device:" << device.name() << '(' << device.address().toString() << ')';
    if(!this->deviceMap.contains(device.name())) {
        this->deviceMap.insert(device.name(), device);
        this->m_deviceNames.append(device.name());
        emit newDevice();
    }
}
