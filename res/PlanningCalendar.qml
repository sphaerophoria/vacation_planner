import QtQuick.Layouts 1.15
import QtQuick 2.15

GridLayout {
    columns: 3
    rows: 4

    rowSpacing: 10
    columnSpacing: 20

    id: calendar

    property int year: 2021
    property date startDate: Date.now()
    property var holidays: []
    property var fixedVacationDays: []
    property var vacationDays: []

    signal clicked(date date)

    Repeater {
        model: 12
        Month {
            month: index
            year: calendar.year
            holidays: parent.holidays.filter(date => date.getMonth() == month).map(date => date.getUTCDate())
            fixedVacationDays: parent.fixedVacationDays.filter(date => date.getMonth() == month).map(date => date.getUTCDate())
            vacationDays: parent.vacationDays.filter(date => date.getMonth() == month).map(date => date.getUTCDate())

            onClicked: calendar.clicked(date)
        }
    }
}
