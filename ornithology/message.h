#ifndef MESSAGE_H
#define MESSAGE_H

#include <QJsonDocument>

class Message
{
public:
    enum MessageType {
        Unknown = 0,
        LastResponse,
        ImageResponse,
        SightingResponse,
        SightingIdsResponse,
    };

    Message();
    QString uuid;
    QString species;
    QString image;
    long timestamp;
    QList<QString> ids;
    MessageType type = Unknown;

    static Message parse(QByteArray data);

    static QJsonDocument LastRequest();
    static QJsonDocument SightingIdsRequest();
    static QJsonDocument SightingRequest(QString uuid);
    static QJsonDocument ImageRequest(QString uuid);
};

#endif // MESSAGE_H
