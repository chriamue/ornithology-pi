#ifndef SIGHTING_H
#define SIGHTING_H

#include <QObject>
#include "message.h"

class Sighting: public QObject
{
    Q_OBJECT
    Q_PROPERTY(QString sightingSpecies READ getSpecies NOTIFY sightingChanged)
    Q_PROPERTY(QString sightingUuid READ getUuid NOTIFY sightingChanged)
    Q_PROPERTY(QString sightingImage READ getImage NOTIFY sightingChanged)
    Q_PROPERTY(QDateTime sightingDatetime READ getDatetime NOTIFY sightingChanged)

public:
    Sighting() = default;
    Sighting(Message *message);
    QString getSpecies() const;
    QString getUuid() const;
    QString getImage() const;
    void setImage(QString image);
    QDateTime getDatetime() const;

Q_SIGNALS:
    void sightingChanged();

private:
    Message m_message;
};

#endif // SIGHTING_H
