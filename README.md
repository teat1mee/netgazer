**NetGazer** 🕵️‍♂️📡

English | Русский

**English**

A lightweight, high-performance network packet sniffer written in Rust.
This tool captures live network traffic, decodes standard protocols (Ethernet, IPv4, TCP, UDP), and inspects packet payloads in real-time.
🚀 Key Features

    Raw Packet Capture: Interacts directly with network interfaces using the libpcap library.

    Protocol Decoding:

        Layer 2: Ethernet Frames.

        Layer 3: IPv4 Addresses (Source & Destination).

        Layer 4: TCP/UDP Ports identification.

    Intelligent Filtering: Built-in filters for Web Traffic (HTTP/80, HTTPS/443).

    Payload Inspection: Converts raw binary buffers into human-readable ASCII text, sanitizing non-printable characters for terminal safety.

🛠 Tech Stack

    Language: Rust (Edition 2021)

    Network Capture: pcap crate

    Packet Parsing: etherparse

    Target OS: Linux (Optimized for Fedora/RHEL/Debian)

📦 Prerequisites

To build and run NetGazer, you need the libpcap development headers.
For Fedora: sudo dnf install libpcap-devel
🔨 Usage

    cargo build --release

    sudo ./target/release/netgazer

**Русский**

Легковесный и высокопроизводительный сетевой сниффер на Rust.
Инструмент перехватывает живой сетевой трафик, декодирует стандартные протоколы (Ethernet, IPv4, TCP, UDP) и анализирует содержимое пакетов в реальном времени.
🚀 Ключевые возможности

    Сырой захват пакетов: Прямое взаимодействие с сетевыми интерфейсами через библиотеку libpcap.

    Декодирование протоколов:

        Layer 2 (Канальный): Ethernet кадры.

        Layer 3 (Сетевой): IPv4 адреса (Источник и Назначение).

        Layer 4 (Транспортный): Идентификация портов TCP/UDP.

    Умная фильтрация: Встроенные фильтры для веб-трафика (HTTP/80, HTTPS/443).

    Анализ полезной нагрузки: Преобразование бинарных данных в читаемый ASCII-текст с защитой терминала от спецсимволов.

🛠 Стек технологий

    Язык: Rust (Edition 2021)

    Захват трафика: pcap crate

    Парсинг пакетов: etherparse

    ОС: Linux (Оптимизировано для Fedora/RHEL/Debian)

📦 Зависимости

Для сборки необходимы заголовочные файлы libpcap.
Для Fedora: sudo dnf install libpcap-devel
🔨 Запуск

    cargo build --release

    sudo ./target/release/netgazer

🛡 Disclaimer / Отказ от ответственности

For educational and ethical security auditing purposes only. Unauthorized sniffing of network traffic is illegal. Use responsibly on networks you own or have explicit permission to test.

Только для образовательных целей и этичного аудита безопасности. Несанкционированный перехват трафика является незаконным. Используйте только в сетях, которыми вы владеете или имеете разрешение на тестирование.
