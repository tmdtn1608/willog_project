# willog_project

- 사용한 프레임워크

  - rocket : Restful api 프레임워크.
  - rocket_contrib : rocket 관련 보조 라이브러리
  - serde : 직렬화 및 역직렬화 라이브러리
  - serde_json : serde의 json 구현체 라이브러리
  - sqlx / sqlx-macros : DB 연동 라이브러리
  - tokio : 비동기 프로그래밍 라이브러리
  - syn : rust 코드분석용
  - rustls : rust tls 프로토콜 라이브러리
  - dotenv / dotenv_codegen : .env파일 연동 라이브러리
  - regex : 정규표현식

- 환경설정

  - cargo : rustup override set nightly로 추가 설정
    ```
    cargo 1.77.0-nightly (7bb7b5395 2024-01-20)
    ```
  - mysql 8.0

- 구현내용
  - 그룹추가 : /Regist/Group
  - 장비추가 : /Regist/Device
  - 온도 저장 : /Set/Temperature
  - 장비별 통계 : /GetTemperature/Device
  - 그룹별 통계 : /GetTemperature/Group
- .env 구성
  - DATABASE_URL : 데이터베이스 url
    - mysql://[id]@[address]:[port]/[dbname]
