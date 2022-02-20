#include "message.h"
#include <QJsonObject>
#include <QJsonArray>

Message::Message()
{
}

Message Message::parse(QByteArray data)
{
    //qDebug() << data;
    QJsonDocument doc = QJsonDocument::fromJson(data);
    if(doc.isEmpty() || doc.isNull()) {
        return Message();
    }
    QJsonObject object = doc.object();
    if(object["op"].toString().startsWith("last_response")) {
        QJsonObject last = object["last"].toObject();
        Message message;
        message.type = LastResponse;
        message.uuid = last["uuid"].toString();
        message.timestamp = last["timestamp"].toInt();
        message.species = last["species"].toString();
        return message;
    } else if(object["op"].toString().startsWith("image_response")) {
        //qDebug() << object;
        Message message;
        message.type = ImageResponse;
        message.uuid = object["uuid"].toString();
        message.image = object["base64"].toString();
        return message;
    }
    else if(object["op"].toString().startsWith("sighting_response")) {
        QJsonObject sighting = object["sighting"].toObject();
        Message message;
        message.type = SightingResponse;
        message.uuid = sighting["uuid"].toString();
        message.timestamp = sighting["timestamp"].toInt();
        message.species = sighting["species"].toString();
        return message;
    } else if(object["op"].toString().startsWith("sighting_ids_response")) {
        Message message;
        message.type = SightingIdsResponse;
        for( auto i: object["ids"].toArray()){
            message.ids.append(i.toString());
        }
        return message;
    }
    else {
        Message message;
        return message;
    }
}

QJsonDocument Message::LastRequest()
{
    QJsonObject payload;
    payload["op"] = "last_request";
    return QJsonDocument( payload );
}

QJsonDocument Message::SightingIdsRequest()
{
    QJsonObject payload;
    payload["op"] = "sighting_ids_request";
    return QJsonDocument( payload );
}

QJsonDocument Message::SightingRequest(QString uuid)
{
    QJsonObject payload;
    payload["op"] = "sighting_request";
    payload["uuid"] = uuid;
    return QJsonDocument( payload );
}

QJsonDocument Message::ImageRequest(QString uuid)
{
    QJsonObject payload;
    payload["op"] = "image_request";
    payload["uuid"] = uuid;
    return QJsonDocument( payload );
}
