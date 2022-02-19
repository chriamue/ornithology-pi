#ifndef DEVICESMODEL_H
#define DEVICESMODEL_H

#include <QAbstractListModel>

class DevicesModel : public QAbstractListModel
{
    Q_OBJECT

public:
    explicit DevicesModel(QStringList *devices, QObject *parent = nullptr);

    enum Roles {
        NameRole = Qt::UserRole
    };

    // Basic functionality:
    int rowCount(const QModelIndex &parent = QModelIndex()) const override;

    QVariant data(const QModelIndex &index, int role = Qt::DisplayRole) const override;

    // Editable:
    bool setData(const QModelIndex &index, const QVariant &value,
                 int role = Qt::EditRole) override;

    Qt::ItemFlags flags(const QModelIndex& index) const override;

    virtual QHash<int, QByteArray>roleNames() const override;

private:
    QStringList * devices;
};

#endif // DEVICESMODEL_H
