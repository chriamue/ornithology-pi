#include <QGuiApplication>
#include <QIcon>
#include <QQmlApplicationEngine>
#include <QQmlContext>
#include <QLocale>
#include <QTranslator>
#include "client.h"
#include "device.h"

int main(int argc, char *argv[])
{
    QGuiApplication::setApplicationName("Ornithology");
    QGuiApplication::setOrganizationName("chriamue");
    QGuiApplication app(argc, argv);
    app.setWindowIcon(QIcon(":/favicon.ico"));

    Device device;
    Client client(&device);
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

    engine.rootContext()->setContextProperty("client", &client);
    engine.rootContext()->setContextProperty("device", &device);

    engine.load(url);

    return app.exec();
}
