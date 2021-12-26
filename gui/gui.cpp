#include <QObject>
#include <QQmlApplicationEngine>
#include <QGuiApplication>
#include <QQmlContext>
#include <QDateTime>
#include <QDebug>

#include <memory>
#include "gui.h"

class Backend : public QObject
{
    Q_OBJECT

    Q_PROPERTY(double numVacationDays WRITE setNumVacationDays);
    Q_PROPERTY(QVariantList fixedVacationDays WRITE setFixedVacationDays);
    Q_PROPERTY(QDateTime startDate NOTIFY startDateChanged WRITE setStartDate);
    Q_PROPERTY(std::uint32_t province WRITE setProvince);
    Q_PROPERTY(QVariantList vacationDays MEMBER vacationDays_ NOTIFY vacationDaysChanged);
    Q_PROPERTY(QVariantList holidays MEMBER holidays_ NOTIFY holidaysChanged);
    Q_PROPERTY(QVariantList provinceList MEMBER provinceList_ NOTIFY provinceListChanged);

public:
    Backend(GuiCallbacks callbacks)
        : callbacks_(callbacks)
    {
        updateHolidays();
    }

signals:
    void startDateChanged();
    void vacationDaysChanged();
    void holidaysChanged();
    void provinceListChanged();

private:
    void setNumVacationDays(std::uint16_t numDays) {
        callbacks_.setNumVacationDays(numDays, callbacks_.data);
        updateVacationDays();
    }

    void setFixedVacationDays(QVariantList fixedDays) {
        std::vector<uint64_t> dates;
        dates.reserve(fixedDays.size());

        std::transform(fixedDays.begin(), fixedDays.end(),
                       std::back_inserter(dates), [](const QVariant &date) {
                         return date.toDateTime().toMSecsSinceEpoch();
                       });

        callbacks_.setFixedVacationDays(dates.data(), dates.size(),
                                        callbacks_.data);
        updateVacationDays();
    }

    void setStartDate(QDateTime startDate) {
        callbacks_.setStartDate(startDate.toMSecsSinceEpoch(), callbacks_.data);
        updateVacationDays();
    }

    void setProvince(std::uint32_t province) {
        callbacks_.setProvince(province, callbacks_.data);
        updateVacationDays();
    }

    void updateVacationDays()
    {
        std::uint64_t *vacationDays;
        std::uint64_t numVacationDays;
        callbacks_.getVacationDays(&vacationDays, &numVacationDays, callbacks_.data);


        auto deleter = [callbacks = callbacks_] (std::uint64_t* ptr) {
            callbacks.freeDateList(ptr);
        };
        std::unique_ptr<std::uint64_t, decltype(deleter)> vacationDaysDeleter(vacationDays, deleter);

        vacationDays_.clear();
        vacationDays_.reserve(numVacationDays);

        std::transform(
            vacationDays, vacationDays + numVacationDays,
            std::back_inserter(vacationDays_),
            [] (uint64_t msecs) {
                return QDateTime::fromMSecsSinceEpoch(msecs);
            });


        emit vacationDaysChanged();
    }

    void updateHolidays()
    {
        std::uint64_t *holidays;
        std::uint64_t numHolidays;
        callbacks_.getHolidays(&holidays, &numHolidays, callbacks_.data);


        auto deleter = [callbacks = callbacks_] (std::uint64_t* ptr) {
            callbacks.freeDateList(ptr);
        };
        std::unique_ptr<std::uint64_t, decltype(deleter)> vacationDaysDeleter(holidays, deleter);

        holidays_.clear();

        std::transform(
            holidays, holidays + numHolidays,
            std::back_inserter(holidays_),
            [] (uint64_t msecs) {
                return QDateTime::fromMSecsSinceEpoch(msecs);
            });

        emit holidaysChanged();
    }
    GuiCallbacks callbacks_;
    QVariantList vacationDays_;
    QVariantList holidays_;
    QVariantList provinceList_;
};



struct Gui
{
    Backend backend;

    Gui(GuiCallbacks callbacks)
        : backend(callbacks)
    {}
};

Gui* makeGui(GuiCallbacks callbacks) {
    return new Gui(callbacks);
}

void destroyGui(Gui* gui) {
    delete gui;
}

void exec(Gui* gui) {
    Q_INIT_RESOURCE(res);
    int argc = 0;
    QGuiApplication app(argc, nullptr);
    QQmlApplicationEngine engine;
    engine.rootContext()->setContextProperty("planner", &gui->backend);
    engine.load(QUrl("qrc:/Planner.qml"));
    app.exec();
}


#include "gui.moc"
