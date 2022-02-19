#ifndef CLIENT_H
#define CLIENT_H

#include <QObject>
#include <QtBluetooth/QBluetoothSocket>
#include <QtBluetooth/QBluetoothAddress>
#include "device.h"

class Client : public QObject
{
    Q_OBJECT
        public:
                 explicit Client(Device *device, QObject *parent = nullptr);


public slots:
    void connect(const QString &address);
signals:

private:
    QBluetoothSocket *socket = nullptr;
    Device * device;

};

#endif // CLIENT_H
