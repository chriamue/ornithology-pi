#include <QGuiApplication>
#include <QQmlApplicationEngine>
#include <QQmlContext>
#include <QLocale>
#include <QTranslator>
#include "bluetooth.h"
#include "devicesmodel.h"

int main(int argc, char *argv[])
{
    QGuiApplication app(argc, argv);

    Bluetooth bluetooth;
    QTranslator translator;
    const QStringList uiLanguages = QLocale::system().uiLanguages();
    for (const QString &locale : uiLanguages) {
        const QString baseName = "ornithology_" + QLocale(locale).name();
        if (translator.load(":/i18n/" + baseName)) {
            app.installTranslator(&translator);
            break;
        }
    }

    QQmlApplicationEngine engine;
    const QUrl url(u"qrc:/ornithology/main.qml"_qs);
    QObject::connect(&engine, &QQmlApplicationEngine::objectCreated,
                     &app, [url](QObject *obj, const QUrl &objUrl) {
        if (!obj && url == objUrl)
            QCoreApplication::exit(-1);
    }, Qt::QueuedConnection);

    QStringList dataList = {
        "Item 1",
        "Item 2",
        "Item 3",
        "Item 4"
    };

    DevicesModel deviceList(&dataList);

    engine.rootContext()->setContextProperty("bluetooth", &bluetooth);
    engine.rootContext()->setContextProperty("deviceList", &deviceList);

    engine.load(url);
    engine.rootContext()->findChild<QObject*>("deviceListView")->setProperty("deviceList", QVariant(dataList));

    return app.exec();
}
