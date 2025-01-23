import { useEffect, useState } from "react";
import {
  Button,
  Box,
  Text,
  Flex,
  Spinner,
  useToast,
  IconButton,
  Tooltip,
} from "@chakra-ui/react";
import { FaRegClipboard } from "react-icons/fa"; // Importar el ícono de React Icons
import { useAccount, useAlert } from "@gear-js/react-hooks";
import { useSailsCalls } from "@/app/hooks";
import { web3FromSource } from "@polkadot/extension-dapp";

function Home() {
  const { account } = useAccount();
  const sails = useSailsCalls();
  const alert = useAlert();
  const toast = useToast();
  const [contractState, setContractState] = useState("");
  const [blockHash, setBlockHash] = useState<string | null>(null);
  const [isLoading, setIsLoading] = useState(false);

  // Función para consultar el estado del contrato
  const fetchContractState = async () => {
    if (!sails) {
      console.error("Error: sails no está inicializado.");
      return;
    }

    setIsLoading(true);
    try {
      const response = await sails.query("Query/LastWhoCall");
      console.log("Respuesta del contrato:", response);
      setContractState(response[1]); // Actualiza el estado del contrato
    } catch (error) {
      console.error("Error al consultar el estado del contrato:", error);
    } finally {
      setIsLoading(false);
    }
  };

  // Función para enviar mensajes con métodos al contrato
  const sendMessageWithMethod = async (method: string) => {
    if (!sails) {
      alert.error("SailsCalls is not started!");
      return;
    }

    if (!account) {
      alert.error("Account is not ready");
      return;
    }

    const { signer } = await web3FromSource(account.meta.source);

    try {
      const response = await sails.command(
        method,
        {
          userAddress: account.decodedAddress,
          signer,
        },
        {
          callbacks: {
            onLoad: () =>
              toast({
                title: "Transaction in progress",
                description: "Message is being sent...",
                status: "info",
                duration: 3000,
                isClosable: true,
              }),
            onSuccess: () =>
              toast({
                title: "Transaction successful",
                description: "Message sent successfully!",
                status: "success",
                duration: 3000,
                isClosable: true,
              }),
            onBlock: (blockHash) => {
              toast({
                title: "Transaction included",
                description: `Message is included in block: ${blockHash}`,
                status: "info",
                duration: 3000,
                isClosable: true,
              });
              setBlockHash(blockHash); // Actualiza el blockHash
            },
            onError: () =>
              toast({
                title: "Transaction failed",
                description: "Error while sending message",
                status: "error",
                duration: 3000,
                isClosable: true,
              }),
          },
        }
      );

      console.log("Response: ", Object.keys(response)[0]);
      await fetchContractState(); // Consulta el estado después de enviar el mensaje
    } catch (e) {
      toast({
        title: "Error",
        description: "Error while sending signless account",
        status: "error",
        duration: 3000,
        isClosable: true,
      });
      console.error(e);
    }
  };

  // Función para copiar el blockHash al portapapeles
  const copyToClipboard = () => {
    if (blockHash) {
      navigator.clipboard.writeText(blockHash);
      toast({
        title: "Copied!",
        description: "Block hash has been copied to clipboard.",
        status: "success",
        duration: 2000,
        isClosable: true,
      });
    }
  };

  // Consulta inicial del estado
  useEffect(() => {
    fetchContractState();
  }, [sails]);

  return (
    <Flex
      direction="column"
      align="center"
      p="6"
      gap="6"
      bg="gray.900"
      minH="100vh"
      justify="center"
    >
      {/* Contenedor del estado y block hash */}
      <Box
        w="100%"
        maxW="400px"
        bgGradient="linear(to-r, #6a11cb, #2575fc)"
        color="white"
        borderRadius="xl"
        p="6"
        boxShadow="2xl"
        textAlign="center"
        transition="transform 0.3s ease"
        _hover={{ transform: "scale(1.05)" }}
      >
        <Text fontSize="2xl" fontWeight="bold">
          Contract State
        </Text>
        {isLoading ? (
          <Spinner size="lg" mt="4" />
        ) : (
          <Text fontSize="lg" mt="4">
            {contractState || "No data available"}
          </Text>
        )}
        <Flex align="center" justify="center" mt="4">
          <Text
            fontSize="md"
            bgGradient="linear(to-r, #FF416C, #FF4B2B)"
            bgClip="text"
            fontWeight="bold"
            whiteSpace="nowrap"
            overflow="hidden"
            textOverflow="ellipsis"
            title={blockHash || "No block hash yet"}
            mr="2"
          >
            {blockHash || "No block hash yet"}
          </Text>
          {blockHash && (
            <Tooltip label="Copy to clipboard" hasArrow>
              <IconButton
                aria-label="Copy block hash"
                icon={<FaRegClipboard />} // Ícono de React Icons
                size="sm"
                onClick={copyToClipboard}
              />
            </Tooltip>
          )}
        </Flex>
      </Box>

      {/* Botones Ping y Pong con gradientes */}
      <Flex gap="6">
        <Button
          size="lg"
          px="8"
          fontSize="lg"
          boxShadow="lg"
          bgGradient="linear(to-r, #00F260, #0575E6)"
          color="white"
          _hover={{
            bgGradient: "linear(to-r, #0575E6, #00F260)",
            transform: "translateY(-2px)",
          }}
          onClick={async () => await sendMessageWithMethod("Ping/Ping")}
        >
          Send Ping
        </Button>
        <Button
          size="lg"
          px="8"
          fontSize="lg"
          boxShadow="lg"
          bgGradient="linear(to-r, #FC466B, #3F5EFB)"
          color="white"
          _hover={{
            bgGradient: "linear(to-r, #3F5EFB, #FC466B)",
            transform: "translateY(-2px)",
          }}
          onClick={async () => await sendMessageWithMethod("Ping/Pong")}
        >
          Send Pong
        </Button>
      </Flex>
    </Flex>
  );
}

export { Home };
