Суть: создать небольшой сервис, который будет выставлять ордера на биржу при достижении определенных условий (цена ниже или равна либо выше или равна указанного значения). Для тестирования можно использовать binance testnet.

Задание:

- Создать json-rpc api interface, который будет:

— принимать параметры ордеров и сохранять их в БД

— по запросу отдавать параметры ордеров со статусом

— по запросу удалять параметры ордеров по id

- Мониторить пары, по которым есть параметры ордеров
- При достижении условия выставлять лимитный ордер на бирже, отсылать нотификацию пользователю в телеграмм
- Мониторить выставленный ордер на предмет закрытия (если не закрылся сразу при выставлении), при полном исполнении отсылать еще одну нотификацию пользователю и считать это правило отработанным
- Покрыть код тестами