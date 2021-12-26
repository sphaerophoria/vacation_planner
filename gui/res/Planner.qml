import QtQuick.Window 2.15
import QtQuick 2.15
import QtQuick.Controls 2.15
import QtQuick.Controls 1.4
import QtQuick.Layouts 1.15

import "./util.js" as Util

Window {
    visible: true

    width: content.width
    height: content.height
    minimumHeight: height
    maximumHeight: height
    minimumWidth: width
    maximumWidth: width

    title: "Vaction Planner"

    Dialog {
        id: startDatePicker
        standardButtons: Dialog.Ok
        anchors.centerIn: parent

        contentItem: Calendar {
            id: startDateCalendar
            selectedDate: new Date(Date.now())
        }
    }

    Binding {
        target: planner
        property: "startDate"
        value: startDateCalendar.selectedDate
    }

    Binding {
        target: planner
        property: "fixedVacationDays"
        value: calendar.fixedVacationDays
    }

    Binding {
        target: planner
        property: "numVacationDays"
        value: numVacationDaysBox.value
    }

    ColumnLayout {
        id: content

        Layout.fillWidth: true

        PlanningCalendar {
            id: calendar
            visible: true
            vacationDays: planner.vacationDays
            fixedVacationDays: []
            holidays: planner.holidays

            onClicked: {
                // Need to create a temporary and re-assign to trigger
                // signal for re-assignment
                var arr = fixedVacationDays

                const index = arr.map(Number).indexOf(+date)
                if (index == -1) {
                    arr.push(date)
                }
                else {
                    arr.splice(index, 1)
                }

                fixedVacationDays = arr
            }
        }

        GridLayout {
            id: settings
            columns: 4

            Layout.fillWidth: true

            Label {
                text: "Number of vacation days: "
            }

            SpinBox {
                id: numVacationDaysBox
                value: 14
            }


            Label {
                text: "Province: "
            }
            ComboBox {

            }

            Button {
                id: startDateButton
                Layout.columnSpan: 2
                text: "Start date: " + Util.monthIndexToName(startDateCalendar.selectedDate.getMonth()) + " " + startDateCalendar.selectedDate.getDate()
                onClicked: startDatePicker.open()
            }
        }

    }
}
