let zdc = ../../dhall-compose/master/ZimplerDockerCompose.dhall
        ? ../dhall-compose/ZimplerDockerCompose.dhall
        ? https://raw.githubusercontent.com/Zimpler/dhall-compose/master/ZimplerDockerCompose.dhall
          using [ { header = "Authorization" , value = "token ${env:GITHUB_TOKEN as Text}" } ]

let pgName = "{{project-name}}"

let pgUser = "postgres"

let pgPwd = "password"

let deps = [ "postgres" ]

let service =
      zdc.mkZSvc
        "{{project-name}}"
        "{{project-name}}"
        "/usr/local/bin/server-cmd -m"
        deps
        [ zdc.mkEnvVar "POSTGRES_HOST" "service-postgres"
        , zdc.mkEnvVar "POSTGRES_DB" pgName
        , zdc.mkEnvVar "POSTGRES_USER" pgUser
        , zdc.mkEnvVar "POSTGRES_PASSWORD" pgPwd
        ]

let service-postgres =
      let envvars =
            [ zdc.mkEnvVar "POSTGRES_DB" pgName
            , zdc.mkEnvVar "POSTGRES_USER" pgUser
            , zdc.mkEnvVar "POSTGRES_PASSWORD" pgPwd
            ]
      in zdc.mkZDb "postgres" "postgres:13.1" envvars ([] : List zdc.ZVolFun)

in  { services = [ service ]
    , dbs = [ service-postgres ]
    , composeServices = [] : List zdc.CSvc
    }
