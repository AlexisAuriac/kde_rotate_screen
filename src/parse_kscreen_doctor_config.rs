use std::io::Read;

use anyhow::Result;
use serde::Deserialize;

// unused properties are commented
#[derive(Deserialize)]
pub struct KScreenDoctorOutput {
    // brightness
    // clones
    // connected
    // currentModeId
    // edrPolicy
    // enabled
    // followPreferredMode
    // hdr
    // iccProfilePath
    // icon
    // id
    // maxBpc
    // modes
    pub name: String,
    // overscan
    // pos
    // preferredModes
    // priority
    // replicationSource
    pub rotation: u8,
    // scale
    // sdr
    // size
    // sizeMM
    // type
    // vrrPolicy
    // wcg
}

// unused properties are commented
#[derive(Deserialize)]
pub struct KScreenDoctorConfig {
    // features
    pub outputs: Vec<KScreenDoctorOutput>,
    // screen
    // tabletModeAvailable
    // tabletModeEngaged
}

pub fn parse_kscreen_doctor_config<R>(r: &mut R) -> Result<KScreenDoctorConfig>
where
    R: Read,
{
    Ok(serde_json::from_reader(r)?)
}

#[cfg(test)]
mod tests {
    use std::io::Cursor;

    use super::*;

    static EXAMPLE_CONFIG: &str = r#"{
  "features": 255,
  "outputs": [
    {
      "brightness": 0.45,
      "clones": [],
      "connected": true,
      "currentModeId": "1",
      "edrPolicy": 1,
      "enabled": true,
      "followPreferredMode": false,
      "hdr": true,
      "iccProfilePath": "",
      "icon": "",
      "id": 1,
      "maxBpc": 0,
      "modes": [
        {
          "id": "1",
          "name": "2560x1600@300",
          "refreshRate": 300,
          "size": {
            "height": 1600,
            "width": 2560
          }
        },
        {
          "id": "10",
          "name": "1280x800@300",
          "refreshRate": 300,
          "size": {
            "height": 800,
            "width": 1280
          }
        },
        {
          "id": "11",
          "name": "1280x720@300",
          "refreshRate": 300,
          "size": {
            "height": 720,
            "width": 1280
          }
        },
        {
          "id": "12",
          "name": "1024x768@300",
          "refreshRate": 300,
          "size": {
            "height": 768,
            "width": 1024
          }
        },
        {
          "id": "13",
          "name": "800x600@300",
          "refreshRate": 300,
          "size": {
            "height": 600,
            "width": 800
          }
        },
        {
          "id": "14",
          "name": "640x480@300",
          "refreshRate": 300,
          "size": {
            "height": 480,
            "width": 640
          }
        },
        {
          "id": "15",
          "name": "1600x1200@60",
          "refreshRate": 59.86899948120117,
          "size": {
            "height": 1200,
            "width": 1600
          }
        },
        {
          "id": "16",
          "name": "1280x1024@60",
          "refreshRate": 59.89500045776367,
          "size": {
            "height": 1024,
            "width": 1280
          }
        },
        {
          "id": "17",
          "name": "1024x768@60",
          "refreshRate": 59.91999816894531,
          "size": {
            "height": 768,
            "width": 1024
          }
        },
        {
          "id": "18",
          "name": "1920x1200@60",
          "refreshRate": 59.8849983215332,
          "size": {
            "height": 1200,
            "width": 1920
          }
        },
        {
          "id": "19",
          "name": "1280x800@60",
          "refreshRate": 59.810001373291016,
          "size": {
            "height": 800,
            "width": 1280
          }
        },
        {
          "id": "2",
          "name": "2560x1600@240",
          "refreshRate": 240,
          "size": {
            "height": 1600,
            "width": 2560
          }
        },
        {
          "id": "20",
          "name": "2560x1440@60",
          "refreshRate": 59.96099853515625,
          "size": {
            "height": 1440,
            "width": 2560
          }
        },
        {
          "id": "21",
          "name": "2560x1440@300",
          "refreshRate": 299.8210144042969,
          "size": {
            "height": 1440,
            "width": 2560
          }
        },
        {
          "id": "22",
          "name": "1920x1080@60",
          "refreshRate": 59.9630012512207,
          "size": {
            "height": 1080,
            "width": 1920
          }
        },
        {
          "id": "23",
          "name": "1600x900@60",
          "refreshRate": 59.94599914550781,
          "size": {
            "height": 900,
            "width": 1600
          }
        },
        {
          "id": "24",
          "name": "1600x900@300",
          "refreshRate": 299.7510070800781,
          "size": {
            "height": 900,
            "width": 1600
          }
        },
        {
          "id": "25",
          "name": "1368x768@60",
          "refreshRate": 59.88199996948242,
          "size": {
            "height": 768,
            "width": 1368
          }
        },
        {
          "id": "26",
          "name": "1368x768@300",
          "refreshRate": 299.7130126953125,
          "size": {
            "height": 768,
            "width": 1368
          }
        },
        {
          "id": "27",
          "name": "1280x720@60",
          "refreshRate": 59.85499954223633,
          "size": {
            "height": 720,
            "width": 1280
          }
        },
        {
          "id": "3",
          "name": "2560x1600@60",
          "refreshRate": 60,
          "size": {
            "height": 1600,
            "width": 2560
          }
        },
        {
          "id": "4",
          "name": "1920x1200@300",
          "refreshRate": 300,
          "size": {
            "height": 1200,
            "width": 1920
          }
        },
        {
          "id": "5",
          "name": "1920x1080@300",
          "refreshRate": 300,
          "size": {
            "height": 1080,
            "width": 1920
          }
        },
        {
          "id": "6",
          "name": "1600x1200@300",
          "refreshRate": 300,
          "size": {
            "height": 1200,
            "width": 1600
          }
        },
        {
          "id": "7",
          "name": "1680x1050@300",
          "refreshRate": 300,
          "size": {
            "height": 1050,
            "width": 1680
          }
        },
        {
          "id": "8",
          "name": "1280x1024@300",
          "refreshRate": 300,
          "size": {
            "height": 1024,
            "width": 1280
          }
        },
        {
          "id": "9",
          "name": "1440x900@300",
          "refreshRate": 300,
          "size": {
            "height": 900,
            "width": 1440
          }
        }
      ],
      "name": "eDP-1",
      "overscan": 0,
      "pos": {
        "x": 0,
        "y": 0
      },
      "preferredModes": [
        "1",
        "2",
        "3"
      ],
      "priority": 1,
      "replicationSource": 0,
      "rotation": 1,
      "scale": 1.3,
      "sdr-brightness": 150,
      "size": {
        "height": 1600,
        "width": 2560
      },
      "sizeMM": {
        "height": 206,
        "width": 329
      },
      "type": 7,
      "vrrPolicy": 2,
      "wcg": true
    }
  ],
  "screen": {
    "currentSize": {
      "height": 1231,
      "width": 1969
    },
    "id": 0,
    "maxActiveOutputsCount": 1,
    "maxSize": {
      "height": 64000,
      "width": 64000
    },
    "minSize": {
      "height": 0,
      "width": 0
    }
  },
  "tabletModeAvailable": false,
  "tabletModeEngaged": false
}"#;

    #[test]
    fn test_parse() -> Result<()> {
        let mut r = Cursor::new(EXAMPLE_CONFIG);

        let cfg = parse_kscreen_doctor_config(&mut r)?;

        assert_eq!(1, cfg.outputs.len());
        let output = &cfg.outputs[0];

        assert_eq!("eDP-1", output.name);
        assert_eq!(1, output.rotation);

        Ok(())
    }
}
