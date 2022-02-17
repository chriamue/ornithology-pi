#include <QtBluetooth/QBluetoothDeviceDiscoveryAgent>
#include <QtBluetooth/QBluetoothServiceDiscoveryAgent>
#include <QtBluetooth/QBluetoothSocket>
#include <QJsonObject>
#include <QJsonDocument>
#include <QDateTime>

#include "bluetoothform.h"
#include "message.h"
#include "sightingform.h"
#include "ui_bluetoothform.h"

BluetoothForm::BluetoothForm(QWidget *parent) :
    QWidget(parent),
    ui(new Ui::BluetoothForm)
{
    ui->setupUi(this);
}

BluetoothForm::~BluetoothForm()
{
    delete ui;
}

void BluetoothForm::on_pushButton_clicked()
{
    ui->deviceList->show();
    ui->deviceList->clear();
    this->deviceMap.clear();
    this->startDeviceDiscovery();
}

void BluetoothForm::startDeviceDiscovery()
{
    deviceDiscoveryAgent = new QBluetoothDeviceDiscoveryAgent(this);
    connect(deviceDiscoveryAgent, SIGNAL(deviceDiscovered(QBluetoothDeviceInfo)),
            this, SLOT(deviceDiscovered(QBluetoothDeviceInfo)));

    // Start a discovery
    deviceDiscoveryAgent->start();
}

void BluetoothForm::deviceDiscovered(const QBluetoothDeviceInfo &device)
{
    qDebug() << "Found new device:" << device.name() << '(' << device.address().toString() << ')';
    if(!this->deviceMap.contains(device.name())) {
        this->ui->deviceList->addItem(device.name());
        this->deviceMap.insert(device.name(), device);
    }
    if(device.name().startsWith("ornithology-pi", Qt::CaseInsensitive)){
        //if(device.name().startsWith("raspberrypi", Qt::CaseInsensitive)){
        this->ui->deviceList->addItem(device.name());
        this->deviceMap.insert(device.name(), device);
        deviceDiscoveryAgent->stop();
        qDebug() << "device is active:"  << deviceDiscoveryAgent->isActive();
        delete deviceDiscoveryAgent;
    }
}

void BluetoothForm::connectDevice(const QBluetoothDeviceInfo &device)
{
    qDebug() << "connecting to "
             << device.name() << device.address().toString();
    QBluetoothUuid uuid("00000000-0000-0000-000f-00dc0de00001");
    if (socket)
        return;

    socket = new QBluetoothSocket(QBluetoothServiceInfo::RfcommProtocol);
    connect(socket, &QBluetoothSocket::readyRead, this, &BluetoothForm::on_clientReady);
    connect(socket, &QBluetoothSocket::errorOccurred, this, &BluetoothForm::on_socketError);
    qDebug() << "Create socket";
    socket->connectToService(device.address(), uuid);
    qDebug() << "ConnectToService done";
    ui->status->setText("connected to " + device.name() + " " + device.address().toString());
}

void BluetoothForm::on_clientReady()
{
    ui->commandLinkButton->setEnabled(true);
    QByteArray chunk = socket->readAll();
    if(chunk.startsWith("{")){
        currentLine.clear();
    }
    this->currentLine += chunk;

    Message message = Message::parse(currentLine);
    if(message.type == Message::MessageType::LastResponse || message.type == Message::MessageType::SightingResponse) {
        auto sighting = new SightingForm(message, this);
        sightings.insert(message.uuid, sighting);
        auto item = new QListWidgetItem();

        item->setSizeHint(sighting->sizeHint());
        ui->lastList->addItem(item);
        ui->lastList->setItemWidget(item, sighting);
        connect(sighting, &SightingForm::viewClicked, this, &BluetoothForm::on_viewClicked);
    }
    else if(message.type == Message::MessageType::ImageResponse) {
        sightings[message.uuid]->setPreview(message.image);
        QPixmap pixmap;
        pixmap.loadFromData(QByteArray::fromBase64(message.image.remove("data:image/jpeg;").toUtf8()));
        ui->preview->setPixmap(pixmap);
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

void BluetoothForm::on_socketError(QBluetoothSocket::SocketError error)
{
    QString s = QVariant::fromValue(error).toString();
    ui->status->setText(s);
}

void BluetoothForm::on_deviceList_itemClicked(QListWidgetItem *item)
{
    deviceDiscoveryAgent->stop();
    connectDevice(this->deviceMap[item->text()]);
    ui->deviceList->hide();
}

void BluetoothForm::on_commandLinkButton_clicked()
{
    QByteArray text = Message::SightingIdsRequest().toJson(QJsonDocument::Compact);
    qDebug() << text;
    socket->write(text);
}

void BluetoothForm::on_viewClicked(QString uuid)
{
    QByteArray text = Message::ImageRequest(uuid).toJson(QJsonDocument::Compact);
    qDebug() << text;
    socket->write(text);
}
