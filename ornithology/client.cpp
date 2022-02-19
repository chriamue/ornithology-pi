#include "client.h"

Client::Client(Device* device, QObject *parent)
    : device(device), QObject{parent}
{

}

void Client::connect(const QString &address)
{
    device->setCurrent(address);
    qDebug() << "connecting to "
             <<address;
    QBluetoothUuid uuid("00000000-0000-0000-000f-00dc0de00001");
    if (socket)
        return;

    socket = new QBluetoothSocket(QBluetoothServiceInfo::RfcommProtocol);
}
