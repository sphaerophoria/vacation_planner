import QtQuick.Window 2.15
import QtQuick 2.15
import QtQuick.Controls 1.4
import QtQuick.Layouts 1.15
import VacationPlanner 1.0

Window {
    visible: true
    width: 400
    height: 600

    VacationPlanner {
        id: planner

        numVacationDays: numVacationDays.value
        startDate: calendar.startDate
        fixedVacationDays: calendar.fixedVacationDays
    }

    ColumnLayout {
        anchors.fill: parent
        anchors.leftMargin: 5
        anchors.rightMargin: 5
        anchors.topMargin: 5
        anchors.bottomMargin: 5

        Layout.fillWidth: true

        PlanningCalendar {
            id: calendar
            visible: true
            startDate: new Date(2021, 01, 01)
            vacationDays: planner.vacationDays
            fixedVacationDays: []
            holidays: planner.holidays

            onClicked: {
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
                id: numVacationDays
                value: 14
            }

            Label {
                text: "Province: "
            }
            ComboBox {

            }
        }

    }
}
