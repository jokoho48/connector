class cfgPatches {
    class Connector_Server {
        units[] = {};
        weapons[] = {};
        requiredVersion = 2.0;
        author = "jokoho48";
        requiredAddons[] = {"A3_Ui_F", "A3_Ui_F_Orange", "A3_Ui_F_Oldman", "A3_Data_F_AoW_Loadorder", "A3_Data_F_Mod_Loadorder"};
    };
};
class CfgMainMenuSpotlight {
    class Server {
        text = "";
        textIsQuote = 0;
        picture = "\z\connector\addons\connector\logo_ca.paa";
        action = "connectToServer parseSimpleArray (('connector' callExtension ['resolve', [0]]) select 0);";
        actionText = "Connect to the Server!";
        condition = "true";
    };
    class AoW_Showcase_AoW {
        condition = "false";
    };
    class AoW_Showcase_Future {
        condition = "false";
    };
    class ApexProtocol {
        condition = "false";
    };
    class Bootcamp {
        condition = "false";
    };
    class Contact_Campaign {
        condition = "false";
    };
    class EastWind {
        condition = "false";
    };
    class gm_campaign_01 {
        condition = "false";
    };
    class OldMan {
        condition = "false";
    };
    class Orange_Campaign {
        condition = "false";
    };
    class Orange_CampaignGerman {
        condition = "false";
    };
    class Orange_Showcase_IDAP {
        condition = "false";
    };
    class Orange_Showcase_LoW {
        condition = "false";
    };
    class Showcase_TankDestroyers {
        condition = "false";
    };
    class SP_FD14 {
        condition = "false";
    };
    class Tacops_Campaign_01 {
        condition = "false";
    };
    class Tacops_Campaign_02 {
        condition = "false";
    };
    class Tacops_Campaign_03 {
        condition = "false";
    };
    class Tanks_Campaign_01 {
        condition = "false";
    };
};

class CfgFunctions {
    class JK {
        class Connector {
            class preStart {
                file = "\z\connector\addons\connector\fn_preStart.sqf";
                preStart = 1;
                preInit = 1;
            };
        };
    };
};