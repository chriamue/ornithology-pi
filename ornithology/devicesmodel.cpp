#include "devicesmodel.h"

DevicesModel::DevicesModel(QStringList* devices, QObject *parent)
    : QAbstractListModel(parent)
{
    this->devices = devices;
}

int DevicesModel::rowCount(const QModelIndex &parent) const
{
    qDebug() << devices->count();
    return devices->count();
}

QVariant DevicesModel::data(const QModelIndex &index, int role) const
{
    if (index.row() < rowCount())
        switch (role) {
        default: return devices->at(index.row());
    }
    return QVariant();
}

bool DevicesModel::setData(const QModelIndex &index, const QVariant &value, int role)
{
    if (data(index, role) != value) {
        // FIXME: Implement me!
        emit dataChanged(index, index, QVector<int>() << role);
        return true;
    }
    return false;
}

Qt::ItemFlags DevicesModel::flags(const QModelIndex &index) const
{
    if (!index.isValid())
        return Qt::NoItemFlags;

    return Qt::ItemIsEditable; // FIXME: Implement me!
}

QHash<int, QByteArray> DevicesModel::roleNames() const
{
    static const QHash<int, QByteArray> roles {
        { NameRole, "name" },
    };
    return roles;
}
