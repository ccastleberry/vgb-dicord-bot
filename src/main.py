
from loguru import logger
import os

import discord
from dotenv import load_dotenv

load_dotenv()
TOKEN = os.getenv('DISCORD_TOKEN')
GUILD = os.getenv('DISCORD_GUILD')


intents = discord.Intents.default()

client = discord.Client(intents=intents)

@client.event
async def on_ready():
    for guild in client.guilds:
        if guild.name == GUILD:
            break

    logger.debug(
        f'{client.user} is connected to the following guild:\n'
        f'{guild.name}(id: {guild.id})'
    )

    members = '\n - '.join([member.name for member in guild.members])
    logger.debug(f'Guild Members:\n - {members}')

@client.event
async def on_message(msg: discord.Message):
    if msg.author == client.user:
        return

    logger.debug(msg.guild)
    logger.debug(msg.channel)
    logger.debug(msg.author)
    logger.debug(msg.content)
    logger.debug(msg.mentions)

    if msg.content == 'ping':
        await msg.channel.send("pong")

    if msg.author.name == "BDrans" and "ffxiv" in msg.content:
        await msg.channel.send("https://www.tiktok.com/t/ZTRUMn53p/?k=1")

    """if msg.author.name == "WastinPplJuice":
        await msg.channel.send("I said no samsies!")"""

    mention_names = [x.name for x in msg.mentions]
    if "cody19" in mention_names:
        await msg.channel.send("poop poop taco poop")

    mentions: list[discord.Member] = msg.mentions
    for mention in mentions:
        logger.debug(mention)



def main():
    client.run(TOKEN)


if __name__ == '__main__':
    main()