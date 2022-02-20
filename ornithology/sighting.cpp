#include "sighting.h"

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
