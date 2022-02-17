#include "sightingform.h"
#include "ui_sightingform.h"

SightingForm::SightingForm(Message message, QWidget *parent) :
    QWidget(parent),
    message(message),
    ui(new Ui::SightingForm)
{
    ui->setupUi(this);
    ui->uuid->hide();
    ui->uuid->setText(message.uuid);
    ui->species->setText(message.species);
    QDateTime dt = QDateTime::fromSecsSinceEpoch(message.timestamp);
    ui->timeEdit->setTime( dt.time());
}

SightingForm::~SightingForm()
{
    delete ui;
}

void SightingForm::setPreview(QString data)
{
    QPixmap pixmap;
    pixmap.loadFromData(QByteArray::fromBase64(data.remove("data:image/jpeg;").toUtf8()));
    ui->pushButton->setIcon(QIcon(pixmap));
    ui->pushButton->setIconSize(ui->pushButton->size());
}

void SightingForm::on_commandLinkButton_2_clicked()
{
    emit viewClicked(this->message.uuid);
}

void SightingForm::on_pushButton_clicked()
{
    emit viewClicked(this->message.uuid);
}

