#ifndef CLIENT_H
#define CLIENT_H

#include <QObject>
#include <QtBluetooth/QBluetoothSocket>
#include <QtBluetooth/QBluetoothAddress>
#include "device.h"
#include "sighting.h"

class Client : public QObject
{
    Q_OBJECT
    Q_PROPERTY(bool socketError READ hasSocketError)
    Q_PROPERTY(QString update READ getUpdate WRITE setUpdate NOTIFY updateChanged)
    Q_PROPERTY(QString picture READ picture NOTIFY pictureUpdated)
    Q_PROPERTY(QVariant sightingsList READ getSightings NOTIFY sightingsUpdated)

        public:
                 explicit Client(Device *device, QObject *parent = nullptr);
    bool hasSocketError() const;
    QVariant getSightings();
    QString picture();
    QString getUpdate();

public slots:
    void connect(const QString &address);
    void disconnect();
    void requestSightingIds();
    void removeSighting(const QString &uuid);
    void loadImage(const QString &uuid);

private slots:
    void on_dataReady();
    void on_socketError(QBluetoothSocket::SocketError error);

Q_SIGNALS:
    void pictureUpdated();
    void sightingsUpdated();
    void updateChanged();

private:
    void setUpdate(const QString &message);
    QBluetoothSocket *socket = nullptr;
    Device * device;
    QByteArray currentLine;
    QList<Sighting *> m_sightings;
    bool idsRequested = false;
    QString m_message;
    QString m_picture;

    void sortSightings();
    Sighting * getSighting(QString uuid);

};

#endif // CLIENT_H
