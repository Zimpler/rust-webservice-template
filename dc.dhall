let zdc = ../dhall-compose/ZimplerDockerCompose.dhall
        ? https://raw.githubusercontent.com/Zimpler/dhall-compose/master/ZimplerDockerCompose.dhall
          using [ { header = "Authorization" , value = "token ${env:GITHUB_TOKEN as Text}" } ]

let service = ./service.dhall

let svcs = zdc.assignServices service.services

let dbs = zdc.assignDbs service.dbs

let vols = zdc.zToComposeFiles svcs dbs

in  zdc.mkCompose
      (zdc.zSvcsToComposeSvcs svcs # zdc.zDbsToComposeSvcs dbs # service.composeServices)
      vols
