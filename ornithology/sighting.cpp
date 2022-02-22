#include "sighting.h"
#include <QPixmap>
#include <QBuffer>

Sighting::Sighting(Message *message)
{
    m_message.uuid = message->uuid;
    m_message.species = message->species;
    m_message.timestamp = message->timestamp;
}

QString Sighting::getSpecies() const
{
    return m_message.species;
}

QString Sighting::getUuid() const
{
    return m_message.uuid;
}

QString Sighting::getImage() const
{
    return m_message.image;
}

void Sighting::setImage(QString image)
{
    m_message.image = image;
    emit sightingChanged();
}

QDateTime Sighting::getDatetime() const
{
    return QDateTime::fromSecsSinceEpoch(m_message.timestamp);
}
