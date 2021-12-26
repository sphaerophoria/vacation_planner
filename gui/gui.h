#ifdef __cplusplus
extern "C" {
#endif
#include "stdint.h"

typedef struct Gui Gui;

typedef struct GuiCallbacks {
    const void* data;
    void (*setNumVacationDays)(uint16_t numDays, const void* data);
    void (*setFixedVacationDays)(const uint64_t* msecsSinceEpochs, uint64_t size, const void* data);
    void (*setStartDate)(uint64_t msecsSinceEpoch, const void* data);
    void (*setProvince)(uint32_t province, const void* data);
    void (*getVacationDays)(uint64_t** msecsSinceEpochs, uint64_t* size, const void* data);
    void (*getHolidays)(uint64_t** msecsSinceEpochs, uint64_t* size, const void* data);
    void (*freeDateList)(uint64_t* vacationDays);

} GuiCallbacks;

Gui* makeGui(GuiCallbacks callbacks);
void destroyGui(Gui* gui);

void exec(Gui* gui);

#ifdef __cplusplus
}
#endif
