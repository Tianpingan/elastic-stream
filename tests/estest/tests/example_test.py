# from ducktape.tests.test import Test
# from estest.services.pd import PD
# from ducktape.mark.resource import cluster
# from ducktape.mark import matrix

# from estest.services.range_server import RangeServer
# from estest.services.verifiable_producer import VerifiableProducer
# from estest.services.verifiable_consumer import VerifiableConsumer
# from estest.services.example import Example
# class ExampleTest(Test):
#     def __init__(self, test_context):
#         super(ExampleTest, self).__init__(test_context=test_context)
#     # @cluster(num_nodes=4)
#     # @matrix(rs_count=[1, 3], count=[100, 1000])
#     def test_example(self):
#         pd = PD(self.test_context, num_nodes=3)
#         pd.start()
#         rs = RangeServer(self.test_context, num_nodes=3, pd=pd)
#         rs.start()
#         example = Example(self.test_context, num_nodes=1, pd=pd, replica=1)
#         example.start()

#         pd.clean()
#         rs.clean()
