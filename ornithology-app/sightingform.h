#ifndef SIGHTINGFORM_H
#define SIGHTINGFORM_H

#include <QWidget>
#include "message.h"

namespace Ui {
class SightingForm;
}

class SightingForm : public QWidget
{
    Q_OBJECT

public:
    explicit SightingForm(Message message, QWidget *parent = nullptr);
    ~SightingForm();
    void setPreview(QString data);

signals:
    void viewClicked(QString uuid);

private slots:
    void on_commandLinkButton_2_clicked();

    void on_pushButton_clicked();

private:
    Ui::SightingForm *ui;
    Message message;
};

#endif // SIGHTINGFORM_H
