use mongod::Client;
use mongod::{Bson, Mongo};

#[derive(Debug, Bson, Mongo)]
struct GuildData {
    DiscordServerID: String,
    RoleLinks: [IRole],
    GEXPData: [IGEXP],
    GEXPWhitelist: [String],
    PardonNewGEXPMembers: Boolean,
    GuildID: String,
    GuildMCBotUUID: String,
    GuildMCBotPassword: String,
    LogChannel: String,
    MCPrefix: String,
    Logging: Boolean,
    IsBotOnline: Boolean,
    BotAutoRun: Boolean,
    StaffRole: String,
    StaffPing: Boolean,
}