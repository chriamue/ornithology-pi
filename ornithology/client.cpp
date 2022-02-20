#include <QPixmap>
#include <QBuffer>
#include "client.h"
#include "message.h"
#include "sighting.h"

Client::Client(Device* device, QObject *parent)
    : device(device), QObject{parent}
{

}

bool Client::hasSocketError() const
{
    return (socket && socket->error() != QBluetoothSocket::SocketError::NoSocketError);
}

QVariant Client::getSightings()
{
    return QVariant::fromValue(m_sightings);
}

QString Client::picture()
{
    return this->m_picture;
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
    QObject::connect(socket, &QBluetoothSocket::readyRead, this, &Client::on_dataReady);
    QObject::connect(socket, &QBluetoothSocket::errorOccurred, this, &Client::on_socketError);
    qDebug() << "Create socket";
    socket->connectToService(device->getCurrent().getDevice().address(), uuid);
    qDebug() << "ConnectToService done";
}

void Client::disconnect()
{
    socket->close();
    delete socket;
    socket = nullptr;
}

void Client::requestSightingIds()
{
    if (!socket)
        return;
    QByteArray text = Message::SightingIdsRequest().toJson(QJsonDocument::Compact);
    socket->write(text);
}

void Client::loadImage(const QString &uuid)
{
    if (!socket)
        return;
    QByteArray text = Message::ImageRequest(uuid).toJson(QJsonDocument::Compact);
    socket->write(text);
}

void Client::on_dataReady()
{
    if(!idsRequested) {
        this->requestSightingIds();
        idsRequested = true;
    }
    QByteArray chunk = socket->readAll();
    if(chunk.startsWith("{")){
        currentLine.clear();
    }
    this->currentLine += chunk;

    Message message = Message::parse(currentLine);
    if(message.type == Message::MessageType::LastResponse || message.type == Message::MessageType::SightingResponse) {
        Sighting * sighting = new Sighting(&message);
        m_sightings.append(sighting);
        emit sightingsUpdated();
    }
    else if(message.type == Message::MessageType::ImageResponse) {
        QPixmap pixmap;
        pixmap.loadFromData(QByteArray::fromBase64(message.image.remove("data:image/jpeg;").toUtf8()));

        QByteArray bArray;
        QBuffer buffer(&bArray);
        buffer.open(QIODevice::WriteOnly);
        pixmap.save(&buffer, "JPEG");

        QString image("data:image/jpg;base64,");
        image.append(QString::fromLatin1(bArray.toBase64().data()));
        m_picture = image; //message.image.replace("data:image/jpeg;", "data:image/jpg;base64,");
        emit pictureUpdated();
    }
    else if(message.type == Message::MessageType::SightingIdsResponse) {
        for(auto id: message.ids) {
            QByteArray text = Message::SightingRequest(id).toJson(QJsonDocument::Compact);
            qDebug() << text;
            socket->write(text);
            socket->write(0);
        }
    }
}

void Client::on_socketError(QBluetoothSocket::SocketError error)
{
    QString s = QVariant::fromValue(error).toString();
    qDebug() << s;
}
