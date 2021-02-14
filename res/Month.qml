import Qt.labs.calendar 1.0
import QtQuick 2.15
import QtQuick.Layouts 1.15
import QtQuick.Controls 1.4

import "./util.js" as Util

Column {
    property int month: Calendar.January
    property int year: 2021
    property var holidays: []
    property var fixedVacationDays: []
    property var vacationDays: []

    width: 200
    height: 200

    signal clicked(date date)

    Label {
        id: monthName

        text: {
            "<b>" + Util.monthIndexToName(parent.month) + "</b>"
        }
    }

    DayOfWeekRow {
        id: days
        locale: grid.locale
    }

    MonthGrid {
        id: grid

        month: parent.month
        year: parent.year
        locale: Qt.locale("en_US")
        width: parent.width
        delegate: Rectangle {
            color: {
                if (model.month != grid.month) {
                    return "transparent"
                }
                else if (holidays.includes(model.day)) {
                    return "green"
                }
                else if (fixedVacationDays.includes(model.day)) {
                    return "red"
                }
                else if (vacationDays.includes(model.day)) {
                    return "yellow"
                }
                else {
                    return "transparent"
                }
            }
            width: 20
            height: 20

            Label {
                visible: model.month == grid.month
                text: model.day
            }
        }

        onClicked: parent.clicked(date)
    }
}
